# Fluxo de trabalho

Uma tarefa do kanban = uma issue = uma branch = um Pull Request.

## Ao começar a trabalhar

1. No board do GitHub Projects, mova o card da tarefa para **Fazendo**.
2. Atualize a `main` e crie a branch da tarefa:

```bash
git switch main
git pull
git switch -c feat/m1-1-conductor      # feat/<id-da-tarefa>-<nome-curto>
```

Prefixos: `feat/` funcionalidade, `fix/` correção, `docs/` documentação, `chore/` configuração.

## Durante o trabalho

Faça commits pequenos e frequentes:

```bash
git add -A
git commit -m "feat(game): toca a música e lê a posição do áudio"
```

Formato da mensagem: `tipo(escopo): descrição` (ex.: `fix(core): corrige janela de release`).

## Antes de subir (evita o CI falhar)

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Terminando a tarefa

```bash
git push -u origin feat/m1-1-conductor
```

1. No GitHub, abra o **Pull Request** para a `main`.
2. Na descrição, coloque `Closes #<número da issue>`.
3. Espere o CI ficar verde ✅ (se falhar, corrija, dê push de novo e ele roda outra vez).
4. Faça **Squash and merge** e apague a branch.
5. A issue fecha sozinha e o card vai para **Feito**.
6. Volte para a base e siga para a próxima tarefa:

```bash
git switch main
git pull
```

## Regras rápidas

- Nunca dê push direto na `main`.
- Branch curta: se a tarefa passar de 2 ou 3 dias, divida em duas no kanban.
- `Cargo.lock` vai no commit; `target/` nunca.
- Mudou dependência? `Cargo.toml` e `Cargo.lock` vão juntos no mesmo commit.
