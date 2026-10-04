# Design intent

Why the code works the way it does. These are deliberate decisions: do not "fix" them without asking. When one changes, update this file in the same change. Everything below "Pending decisions" is the exception: those are unconfirmed proposals made by agents, waiting for the maintainer.

## Project-wide principles

1. **Audio drives time.** The song position (via `Conductor`) is the only source of time. Nothing is positioned with frame deltas.
2. **Judgement depends on time only.** Lane curves and rotations are visual; hit detection uses only the note's time.
3. **Charts are data.** Everything a chart can do is JSON; no code in charts.
4. **Configurable.** Judgement windows and score weights are constants/config that are easy to tune.

## core/clock.rs: `SmoothClock`

- The audio backend reports the position once per audio buffer (roughly every 5-20 ms), so reading it per frame gives a staircase. `SmoothClock` fills the gaps using a real-time clock.
- The estimate may run at most `max_drift_ms` (30 ms) ahead of the last audio position. If the audio stalls, the clock **holds** at that limit. It must never snap back: time must never go backwards (there is a test for this).
- When not playing, it returns the raw audio position.

## core/judgement.rs

- `delta_ms = input_time - note_time`. Negative = early, positive = late.
- `judge_press` returns `None` when the press is too early (beyond `miss_ms`) **or too late (beyond `good_ms`)**. A late note is resolved by `is_expired` (delta > `good_ms`), which the game loop turns into a Miss. Returning a Miss from `judge_press` as well would count the same note twice.
- `miss_ms` applies to early presses only.
- Releasing a hold uses windows multiplied by `release_multiplier` (1.5) and always returns a judgement.
- The window numbers come from the osu!mania OD formula, written from memory. They are marked TODO and are NOT verified against the osu! wiki. Do not describe them as official.

## core/score.rs

- Score is 0 to 1,000,000: 700,000 from accuracy and 300,000 from combo (accuracy weighs more).
- Accuracy weights: Perfect 1, Great 2/3, Good 1/3, Miss 0.
- A regular note is 1 judged object; a hold is 2 (head and release).
- Combo value is capped at `COMBO_CAP` (100). Scores above 1,000,000 are reserved for modifiers (Hidden, DT), a future task.
- `ScoreState::counts` is indexed by `Judgement as usize`, so the enum order matters.

## core/timeline.rs: `ScrollTimeline`

- Distance is the area under the speed graph: sum of speed x duration per segment.
- Scroll is **independent of BPM**. Note position needs only the scroll timeline. BPM is needed only to convert time to beats (a later task).
- If nothing is defined at time 0, an implicit point with speed 1.0 is inserted, so notes never freeze.
- Points at the same time: the last one wins. Negative speed is allowed (notes move back).
- Negative times currently return distance 0 (TODO: extend backwards with the first point's speed).

## chart (crates/chart)

- Times in ms from the start of the audio. Coordinates: screen height = 1.0, x right, y down.
- A lane path is a Bézier curve (2 to 4 control points) in the judgement line's local space; the first point is the spawn point, the last is the judged point.
- Between keyframes, the easing of the **following** keyframe applies. `step` holds the previous value until the following keyframe's `time_ms`, then jumps to its value.
- Consecutive path keyframes must have the same number of points, unless the following keyframe uses `step`. This is how a lane changes curve degree.
- The first `timing` point must be at `time_ms = 0`.
- osu!mania compatibility is via a future converter to this JSON; the game never reads `.osu` directly.
- `charts/example/chart.json` must always pass `Chart::from_json`; a test enforces it.

## game/conductor.rs: `Conductor`

- Wraps kira. `time_ms()` is called once per frame and the same value is used for the whole frame.
- `offset_ms` (chart): positive means the chart starts later than the audio. `calibration_ms` (player, M1-9): positive means audio is heard late. `time_ms()` subtracts both.
- kira 0.10: `StaticSoundHandle::pause/resume/stop(Tween)` return `()`. `position()` is in seconds (f64). A stopped sound cannot be restarted: play a new one. Do not rely on dropping a handle to stop a sound; call `stop` first.
- `wall_origin` only feeds `SmoothClock` with differences, so its absolute value does not affect the result.

## game/main.rs

- Run from the repository root (the chart path is relative).
- Early tasks are debug-oriented. The window title and HUD text are placeholders.

## Known false alarms (do not report these)

- `judge_press` ignoring late presses (see judgement above).
- `is_expired` using `good_ms` instead of `miss_ms`.
- Division by zero in `ScoreState::score`: guarded by the `total_objects == 0` early return.
- "CI has no Windows runner": the README's Windows note is about local builds only.

## Pending decisions (agent-proposed, not confirmed)

Entries here were written by agents while working on a task. They are proposals, not settled intent: the maintainer confirms one by moving it into the matching module section above (or rejects it by deleting it). Agents may add entries but must not edit confirmed sections.

Format: `- **<area>: <decision>.** Why: <reason>. Alternatives: <rejected options>. Task: <roadmap id>. Status: proposed`

(none yet)
