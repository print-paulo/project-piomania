# AGENTS.md

Standing instructions for AI coding agents in this repository. Humans: see `README.md` and `docs/`.

## Read first

1. This file (always).
2. `docs/design-intent.md` BEFORE touching, reviewing or documenting any code. It records why each module works the way it does. Anything listed there is intentional: do not report it as a bug and do not "fix" it. If you believe it is wrong, say so as a question and wait; do not change it.
3. `docs/roadmap.md` to see which task is current. Work only on the task you were given.

## Project

Piomania: a PC rhythm game in Rust (Macroquad for window/render, Kira 0.10 for audio). osu!mania-style precision, Phigros-style moving judgement lines, Geometry Dash/ADOFAI-style choreographed effects. 4K first, 7K later.

The maintainer is a student learning Rust. Prefer small, readable diffs and plain idioms. Explain any non-obvious Rust feature in a short comment or in your report.

## Commands (run before declaring a task done)

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Development is on Windows (PowerShell); CI runs the same checks on Linux. Give commands that work in PowerShell.

## Structure and dependency rule

```
crates/core/    rules: clock, judgement windows, scoring, scroll timeline   (NO render/audio deps)
crates/chart/   JSON chart format: structs, parsing, validation             (NO render/audio deps)
crates/game/    executable (package name: piomania): window, input, render, audio
```

- `game` -> `core`, `game` -> `chart`. Never the reverse. `core` must not depend on `chart`.
- Dependency versions live in the root `Cargo.toml` (`[workspace.dependencies]`). Ask before adding or upgrading any dependency.
- `Cargo.lock` is committed. Never commit `target/`. Music files are committed only under `charts/example/` and only if royalty-free.

## Scope rules

- Do ONLY what the task asks. No drive-by refactors, renames, reformatting or "improvements".
- Do not create new files unless the task says so. Edit existing ones.
- Never commit, push, or open PRs unless explicitly asked. Leave changes in the working tree.
- Do not touch `.github/`, dependency lists, or `docs/roadmap.md` unless the task is about them.
- Never delete or weaken a test to make it pass. If a test looks wrong, explain why and ask.
- If the task is ambiguous, or conflicts with `docs/design-intent.md`, stop and ask. Do not guess.

## Verify, do not assume

- External crates: read the real API for the pinned version (docs.rs for `kira` 0.10.x / `macroquad` 0.4, or the crate source in the cargo registry). Never rely on memory. Example of a past mistake: kira's `pause`/`resume`/`stop` return `()`, not `Result`, so `?` on them does not compile.
- Verify claims about this repo by reading the code. Cite `file:line` read in this session, not remembered.
- Never say tests, clippy or fmt "pass" unless you ran them and saw the output. Say "not run" otherwise.
- Removing `#![allow(dead_code)]` makes `-D warnings` fail on unused items. Use per-item `#[allow(dead_code)] // TODO(<task-id>)` for items a later task will use.
- Do not add comments or docs that state unverified facts as verified (for example, the judgement window numbers are from memory and marked TODO).

## Code conventions

- Everything in the repo is in **English**: code, comments, error messages, docs, commit messages.
- Return `Result` instead of panicking outside `main` and tests.
- Compare `f64` in tests with a tolerance (`(a - b).abs() < 1e-9`), never `==`. Sort floats with `total_cmp`.
- New logic gets unit tests. A bug fix gets a test that fails before the fix.
- `///` comments explain why, not what the code already says.
- Time is in milliseconds (`f64`) unless a name says otherwise. Coordinates: screen height = 1.0, y grows downward.

## Git conventions

- `main` is protected. Branches `feat/`, `fix/`, `docs/`, `chore/`, always created from `main`. One roadmap task = one branch = one PR.
- Commit style: `type(scope): description`, e.g. `fix(chart): validate timing origin`.

## Reviews

- Report only problems you verified in the code. For each: `file:line`, what breaks, a concrete failing scenario, severity (bug / risk / style), and your confidence.
- Skip anything covered by `docs/design-intent.md`.
- Review means review: do not modify code unless asked to apply fixes.

## Documentation tasks

- Edit only the file(s) named. Do not rewrite sections you were not asked to change, and do not create long new documents.
- Docs must match the code. If unsure, read the code or ask.
- When a design decision changes, update `docs/design-intent.md` in the same change.

## Final report format

End every task with:
1. Files changed (and confirmation that nothing else was touched).
2. Commands run with their real results (or "not run").
3. Anything you were unsure about or did not verify.
4. Any design-intent conflict you noticed but did not act on.
