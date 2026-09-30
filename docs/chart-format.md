# Formato de chart (versão 1)

Arquivo JSON. Tempos em **milissegundos** relativos ao início do áudio. Coordenadas em unidades onde **altura da tela = 1.0** (x cresce para a direita, y cresce para baixo). O chart de referência está em `charts/example/chart.json`.

## Campos principais

| Campo             | Descrição                                                             |
| ----------------- | --------------------------------------------------------------------- |
| `format_version`  | Sempre `1` por enquanto.                                              |
| `metadata`        | `title`, `artist`, `charter`, `difficulty_name`, `keys` (4 ou 7).     |
| `audio`           | `file` e `offset_ms` (ajuste fixo do áudio).                          |
| `timing`          | Lista de `{ time_ms, bpm }`. Ao menos um ponto, em `0`.               |
| `scroll`          | Lista de `{ time_ms, speed }`. Muda a velocidade das notas. Opcional. |
| `judgement_lines` | Linhas de julgamento com keyframes de posição, rotação e alpha.       |
| `lanes`           | Uma por tecla; cada uma pertence a uma line e tem um `path` animável. |
| `notes`           | `{ time_ms, lane }`; com `end_time_ms` vira hold.                     |
| `events`          | Efeitos: `flash`, `shake`, `palette`. Opcional.                       |

## Judgement lines

Cada keyframe: `time_ms`, `x`, `y`, `rotation_deg` (padrão 0), `alpha` (padrão 1) e `easing`. O estado entre keyframes é interpolado usando o easing do keyframe seguinte.

## Lanes e curvas

Cada lane tem `path`: uma lista de keyframes `{ time_ms, points, easing }`.

- `points` são pontos de uma **curva de Bézier** no espaço local da judgement line: 2 pontos = reta, 3 = quadrática, 4 = cúbica.
- O **primeiro ponto** é onde a nota nasce; o **último** é onde ela é julgada.
- Entre dois keyframes, os pontos são interpolados, e por isso a curva pode mudar durante a música (ambos os keyframes devem ter o mesmo número de pontos ao interpolar).
- `alpha` (opcional) é uma lista de `{ time_ms, alpha, easing }` para a lane aparecer e desaparecer.

Cada nota percorre a curva da sua lane até o ponto final; a posição ao longo da curva vem da distância restante até o julgamento (que depende de BPM e scroll).

## Easing

`linear` (padrão), `in`, `out`, `in_out`, `step`.

## Notas

- Nota simples: `{ "time_ms": 1000, "lane": 2 }`
- Hold: `{ "time_ms": 1000, "lane": 2, "end_time_ms": 1500 }` (cabeça e soltura são julgadas)

## Eventos

```json
{ "time_ms": 3200, "type": "flash",   "duration_ms": 250, "color": [1, 1, 1] }
{ "time_ms": 6400, "type": "palette", "duration_ms": 400, "background": [0.05, 0.02, 0.15] }
{ "time_ms": 9600, "type": "shake",   "duration_ms": 300, "intensity": 0.6 }
```

## Validação

`Chart::from_json` rejeita: `format_version` diferente, `timing` vazio, BPM ≤ 0, `keys` diferente do número de lanes, lane apontando para line inexistente, path com menos de 2 pontos, nota em lane inexistente e hold que termina antes de começar.

## Plano de compatibilidade com osu!mania

Um conversor (crate `tools/osu-convert`, futuro) lerá `.osu` do modo mania e gerará este JSON (notas, holds, BPM e scroll). O jogo não lê `.osu` diretamente, porque o formato do osu! não tem lanes curvas nem efeitos.
