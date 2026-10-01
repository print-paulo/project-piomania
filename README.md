# Piomania

A PC rhythm game that blends the precision of **osu!mania** and **Phigros** with the visual intensity of **Geometry Dash** and **ADOFAI**.

- Notes travel along lanes that can **curve, rotate, fade in/out and move to the beat of the music**
- Moving judgement lines
- Choreographed effects (shake, flash, palette swaps, perspective inversions) synced to the music
- 4K first, 7K later. Hold releases are judged. Score from 0 to 1,000,000

Built in **Rust** (Macroquad to start, with the option to move rendering to wgpu later).

## Getting started

Requires a recent Rust toolchain (install with [rustup](https://rustup.rs)). On Windows you also need the Visual Studio Build Tools with the "Desktop development with C++" workload.

```bash
cargo test                 # core and chart-format tests
cargo run -p piomania      # launches the game (run from the repository root)
```

## Project structure (Cargo workspace)

A **workspace** is a repository with several crates (Rust packages) that share one `Cargo.lock` and one `target/` folder. Each crate builds separately and only sees the crates it declares as dependencies.

```
piomania/
├── Cargo.toml            # workspace root + shared dependency versions
├── crates/
│   ├── core/             # rules: judgement windows and scoring (no render/audio)
│   ├── chart/            # JSON chart format: structs, parsing and validation
│   └── game/             # executable: window, input, rendering, audio
├── charts/example/       # example chart
└── docs/
    ├── chart-format.md   # chart format specification
    ├── roadmap.md        # task backlog by milestone
    └── workflow.md       # git/PR workflow
```

Dependency rule (never the other way around):

```
game  ──►  core
game  ──►  chart
```

`core` and `chart` know nothing about windows or audio. That keeps them testable and makes it possible to swap the renderer later. Future tools (`.osu` converter, chart editor) will be added as new crates under `tools/`.

## Principles

1. **Audio drives time.** The song position (the conductor) is the only source of time; nothing is positioned using frame deltas.
2. **Judgement depends on time only.** Lane curves are visual; hit detection uses only the note's time.
3. **Charts are data.** Everything a chart can do is expressed in JSON, with no code.
4. **Configurable.** Judgement windows and score weights live in constants/config that are easy to tune.

## Git conventions

- Main branch: `main`. Work happens in `feat/`, `fix/`, `docs/` and `chore/` branches.
- Commit style: `feat(core): add release window`.
- One roadmap task = one branch = one PR (even when working solo, it keeps the history clean).
