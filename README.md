# Piomania

Jogo de ritmo para PC que junta a precisão do **osu!mania** e do **Phigros** com os efeitos visuais de **Geometry Dash** e **ADOFAI**.

- Notas em lanes que podem **curvar, rotacionar, aparecer/sumir e se mover ao ritmo da música**
- Judgement lines móveis
- Efeitos coreografados (shake, flash, troca de paleta, inversões) sincronizados com a música
- 4K primeiro, 7K depois. Holds com soltura julgada. Score de 0 a 1.000.000

Feito em **Rust** (Macroquad no início, com a possibilidade de migrar o render para wgpu).

## Como rodar

Precisa de Rust recente (instale com [rustup](https://rustup.rs)).

```bash
cargo test                    # testes do núcleo e do formato de chart
cargo run -p rhythm-game      # abre o jogo (rode da raiz do repositório)
```

## Estrutura (workspace do Cargo)

Um **workspace** é um repositório com vários crates (pacotes Rust) que compartilham o mesmo `Cargo.lock` e a mesma pasta `target/`. Cada crate compila separadamente e só enxerga os que declarar como dependência.

```
rhythm-game/
├── Cargo.toml            # raiz do workspace + versões das dependências
├── crates/
│   ├── core/             # regras: janelas de julgamento e score (sem render/áudio)
│   ├── chart/            # formato de chart JSON: structs, leitura e validação
│   └── game/             # executável: janela, input, render, áudio
├── charts/example/       # chart de exemplo
├── docs/chart-format.md  # especificação do formato
└── KANBAN.md             # backlog inicial
```

Regra de dependência (nunca no sentido contrário):

```
game  ──►  core
game  ──►  chart
```

`core` e `chart` não sabem que existe janela nem áudio. Isso os mantém testáveis e permite trocar o render depois. Ferramentas futuras (conversor de `.osu`, editor de charts) entram como novos crates em `tools/`.

## Princípios

1. **O áudio manda no tempo.** A posição da música (conductor) é a única fonte de tempo; nada é posicionado por delta de frame.
2. **O julgamento depende só de tempo.** A curva das lanes é visual; o acerto usa apenas o tempo da nota.
3. **Charts são dados.** Tudo que o chart pode fazer está em JSON, sem código.
4. **Configurável.** Janelas de julgamento e pesos do score ficam em constantes/config fáceis de ajustar.

## Convenções de git

- Branch principal: `main`. Trabalho em branches `feat/`, `fix/`, `docs/`.
- Commits no estilo: `feat(core): adiciona janela de release`.
- Uma tarefa do kanban = uma branch = um PR (mesmo trabalhando sozinho, o histórico fica limpo).
