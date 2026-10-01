# Workflow

One roadmap task = one issue = one branch = one Pull Request.

## When you start working

1. On the GitHub Projects board, move the task's card to **Doing**.
2. Update `main` and create the task branch:

```bash
git switch main
git pull
git switch -c feat/m1-1-conductor      # feat/<task-id>-<short-name>
```

Prefixes: `feat/` new feature, `fix/` bug fix, `docs/` documentation, `chore/` configuration.

## While working

Make small, frequent commits:

```bash
git add -A
git commit -m "feat(game): play the song and read the audio position"
```

Message format: `type(scope): description` (e.g. `fix(core): correct release window`).

## Before pushing (avoids a failing CI)

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Finishing the task

```bash
git push -u origin feat/m1-1-conductor
```

1. On GitHub, open a **Pull Request** into `main`.
2. In the description, add `Closes #<issue number>`.
3. Wait for CI to turn green ✅ (if it fails, fix it, push again and it runs once more).
4. Use **Squash and merge** and delete the branch.
5. The issue closes by itself and the card moves to **Done**.
6. Go back to `main` and move on to the next task:

```bash
git switch main
git pull
```

## Quick rules

- Never push directly to `main`.
- Keep branches short: if a task takes more than 2–3 days, split it into two roadmap tasks.
- Always create the new branch from `main`, not from another feature branch.
- `Cargo.lock` is committed; `target/` never is.
- Changed a dependency? Commit `Cargo.toml` and `Cargo.lock` together.
