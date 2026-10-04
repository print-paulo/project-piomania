# Roadmap

Suggested board columns: **Backlog → To do → Doing → Review/Test → Done**.
Task prefixes: `M0`, `M1`... (milestones) + number.

## M0 — Foundation (repo and setup)

- [x] M0-1 Cargo workspace with `core`, `chart`, `game`
- [x] M0-2 Judgement windows and scoring with tests
- [x] M0-3 Chart format v1 with parsing and validation
- [x] M0-4 Create the GitHub repository, protect `main` and set up the Projects board
- [x] M0-5 CI on GitHub Actions: `cargo fmt --check`, `cargo clippy`, `cargo test`
- [x] M0-6 Run `cargo run -p piomania` and confirm the window opens

## M1 — 4K MVP (one song, straight lanes)

- [x] M1-1 Conductor: time from the real audio position (choose `kira` or `rodio`)
- [x] M1-2 Play the song and show the time on screen (debug)
- [x] M1-3 BPM/scroll timeline: `time → accumulated distance` table
- [x] M1-4 Render notes falling in 4 straight lanes
- [ ] M1-5 Input (D F J K) with timestamps and judgement of regular notes
- [ ] M1-6 Holds: head, holding and judging the release
- [ ] M1-7 HUD: score, combo, accuracy, judgement feedback
- [ ] M1-8 Results screen
- [ ] M1-9 Audio/input offset calibration

## M2 — Dynamic lanes and lines

- [ ] M2-1 Keyframe interpolation with easings
- [ ] M2-2 Evaluate Bézier curves and precompute a point table per lane
- [ ] M2-3 Notes following the lane curve
- [ ] M2-4 Moving/rotating judgement line (lanes as children of the line)
- [ ] M2-5 Lane alpha (fade in/out)
- [ ] M2-6 Curves that change during the song (interpolate control points)
- [ ] M2-7 Movement tied to the conductor (per beat)

## M3 — Effects

- [ ] M3-1 Chart event system (fire at the right time)
- [ ] M3-2 Flash and camera shake
- [ ] M3-3 Background palette swap
- [ ] M3-4 Post-processing via shaders (inversions, distortions)
- [ ] M3-5 Scene transitions

## M4 — Content and tools

- [ ] M4-1 7K support
- [ ] M4-2 `.osu` (mania) → game JSON converter
- [ ] M4-3 Song select menu
- [ ] M4-4 Persist settings (offset, scroll speed, key bindings)
- [ ] M4-5 Chart editor (large milestone, split into tasks later)

## M5 — Polish and release

- [ ] M5-1 Modifiers (Hidden, DT) and scores above 1,000,000
- [ ] M5-2 Playtests and tuning of judgement windows
- [ ] M5-3 Windows/Linux builds, icon and itch.io/Steam page
- [ ] M5-4 Trailer and screenshots

## Risks to watch

- Audio latency and synchronization (validate early, in M1-1/M1-2)
- Macroquad vs wgpu decision when reaching post-processing (M3-4)
- Music licensing (use only royalty-free songs in examples)
