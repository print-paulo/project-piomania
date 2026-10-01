# Chart format (version 1)

A JSON file. Times are in **milliseconds** relative to the start of the audio. Coordinates use units where **screen height = 1.0** (x grows to the right, y grows downward). The reference chart is `charts/example/chart.json`.

## Main fields

| Field             | Description                                                       |
| ----------------- | ----------------------------------------------------------------- |
| `format_version`  | Always `1` for now.                                               |
| `metadata`        | `title`, `artist`, `charter`, `difficulty_name`, `keys` (4 or 7). |
| `audio`           | `file` and `offset_ms` (fixed audio offset).                      |
| `timing`          | List of `{ time_ms, bpm }`. At least one point, at `0`.           |
| `scroll`          | List of `{ time_ms, speed }`. Changes note speed. Optional.       |
| `judgement_lines` | Judgement lines with position, rotation and alpha keyframes.      |
| `lanes`           | One per key; each belongs to a line and has an animatable `path`. |
| `notes`           | `{ time_ms, lane }`; adding `end_time_ms` turns it into a hold.   |
| `events`          | Effects: `flash`, `shake`, `palette`. Optional.                   |

## Judgement lines

Each keyframe has `time_ms`, `x`, `y`, `rotation_deg` (default 0), `alpha` (default 1) and `easing`. The state between keyframes is interpolated using the easing of the following keyframe.

## Lanes and curves

Each lane has a `path`: a list of keyframes `{ time_ms, points, easing }`.

- `points` are the control points of a **Bézier curve** in the judgement line's local space: 2 points = straight, 3 = quadratic, 4 = cubic.
- The **first point** is where the note spawns; the **last point** is where it is judged.
- Between two keyframes the points are interpolated, so the curve can change during the song (both keyframes must have the same number of points to be interpolated).
- `alpha` (optional) is a list of `{ time_ms, alpha, easing }` that makes the lane fade in and out.

Each note travels along its lane's curve toward the end point; its position along the curve comes from the remaining distance to the judgement (which depends on BPM and scroll).

## Easing

`linear` (default), `in`, `out`, `in_out`, `step`.

## Notes

- Regular note: `{ "time_ms": 1000, "lane": 2 }`
- Hold: `{ "time_ms": 1000, "lane": 2, "end_time_ms": 1500 }` (both the head and the release are judged)

## Events

```json
{ "time_ms": 3200, "type": "flash",   "duration_ms": 250, "color": [1, 1, 1] }
{ "time_ms": 6400, "type": "palette", "duration_ms": 400, "background": [0.05, 0.02, 0.15] }
{ "time_ms": 9600, "type": "shake",   "duration_ms": 300, "intensity": 0.6 }
```

## Validation

`Chart::from_json` rejects: a `format_version` other than 1, an empty `timing`, BPM ≤ 0, `keys` different from the number of lanes, a lane pointing to a missing line, a path with fewer than 2 points, a note in a missing lane, and a hold that ends before it starts.

## osu!mania compatibility plan

A converter (a future `tools/osu-convert` crate) will read osu!mania `.osu` files and generate this JSON (notes, holds, BPM and scroll). The game does not read `.osu` directly, because the osu! format has no curved lanes or effects.
