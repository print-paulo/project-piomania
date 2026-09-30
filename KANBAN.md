# Kanban inicial

Colunas sugeridas: **Backlog → A fazer → Fazendo → Revisão/Teste → Feito**.
Prefixos de tarefa: `M0`, `M1`... (marcos) + número.

## M0 — Fundação (repo e setup)

- [ ] M0-1 Workspace do Cargo com `core`, `chart`, `game`
- [ ] M0-2 Janelas de julgamento e score com testes
- [ ] M0-3 Formato de chart v1 com leitura e validação
- [ ] M0-4 Criar repositório no GitHub, configurar `main` protegida e board do Projects
- [ ] M0-5 CI no GitHub Actions: `cargo fmt --check`, `cargo clippy`, `cargo test`
- [ ] M0-6 Rodar `cargo run -p rhythm-game` e confirmar a janela abrindo

## M1 — MVP 4K (uma música, linha reta)

- [ ] M1-1 Conductor: tempo a partir da posição real do áudio (escolher `kira` ou `rodio`)
- [ ] M1-2 Tocar a música e exibir o tempo na tela (debug)
- [ ] M1-3 Timeline de BPM/scroll: tabela `tempo → distância acumulada`
- [ ] M1-4 Renderizar notas caindo em 4 lanes retas
- [ ] M1-5 Input (D F J K) com timestamp e julgamento das notas simples
- [ ] M1-6 Holds: cabeça, segurar e julgar a soltura
- [ ] M1-7 HUD: score, combo, precisão, feedback de julgamento
- [ ] M1-8 Tela de resultado
- [ ] M1-9 Calibração de offset de áudio/input

## M2 — Lanes e linhas dinâmicas

- [ ] M2-1 Interpolação por keyframes com easings
- [ ] M2-2 Avaliar Bézier e pré-calcular tabela de pontos por lane
- [ ] M2-3 Notas seguindo a curva da lane
- [ ] M2-4 Judgement line móvel/rotacionada (lanes como filhas da line)
- [ ] M2-5 Alpha das lanes (aparecer/sumir)
- [ ] M2-6 Curvas que mudam durante a música (interpolar pontos de controle)
- [ ] M2-7 Movimento ligado ao conductor (por beat)

## M3 — Efeitos

- [ ] M3-1 Sistema de eventos do chart (disparo no tempo certo)
- [ ] M3-2 Flash e shake de câmera
- [ ] M3-3 Troca de paleta do fundo
- [ ] M3-4 Pós-processamento via shaders (inversões, distorções)
- [ ] M3-5 Transições de cenário

## M4 — Conteúdo e ferramentas

- [ ] M4-1 Suporte a 7K
- [ ] M4-2 Conversor `.osu` (mania) → JSON do jogo
- [ ] M4-3 Menu de seleção de músicas
- [ ] M4-4 Persistir configurações (offset, scroll speed, teclas)
- [ ] M4-5 Editor de charts (marco grande, dividir em tarefas depois)

## M5 — Polimento e lançamento

- [ ] M5-1 Modificadores (Hidden, DT) e score acima de 1.000.000
- [ ] M5-2 Playtests e ajuste das janelas de julgamento
- [ ] M5-3 Build para Windows/Linux, ícone e página no itch.io/Steam
- [ ] M5-4 Trailer e capturas

## Riscos a acompanhar

- Latência e sincronização de áudio (validar cedo, no M1-1/M1-2)
- Escolha entre Macroquad e wgpu ao chegar no pós-processamento (M3-4)
- Licença das músicas (usar apenas músicas livres nos exemplos)
