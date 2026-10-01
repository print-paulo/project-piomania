//! Entry point. For now: opens the window, loads the example chart
//! and draws the judgement line + straight lanes (just to validate the setup).

mod conductor;

use macroquad::prelude::*;
use rhythm_chart::Chart;
use rhythm_core::JudgementWindows;

fn window_conf() -> Conf {
    Conf {
        window_title: "piomania (work in progress)".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let text = std::fs::read_to_string("charts/example/chart.json")
        .expect("run the game from the repository root (cargo run -p piomania)");
    let chart = Chart::from_json(&text).expect("invalid chart");
    let windows = JudgementWindows::default();

    loop {
        clear_background(Color::new(0.04, 0.04, 0.07, 1.0));

        // Coordinate unit: screen height = 1.0 (same as the chart format).
        let unit = screen_height();
        let line = &chart.judgement_lines[0].keyframes[0];
        let (lx, ly) = (line.x * screen_width(), line.y * unit);

        for lane in &chart.lanes {
            let p = &lane.path[0].points;
            let start = p[0];
            let end = p[p.len() - 1];
            draw_line(
                lx + start[0] * unit,
                ly + start[1] * unit,
                lx + end[0] * unit,
                ly + end[1] * unit,
                2.0,
                DARKGRAY,
            );
        }
        draw_line(lx - 0.3 * unit, ly, lx + 0.3 * unit, ly, 4.0, WHITE);

        draw_text(
            format!(
                "{} - {}  |  {} notes ({} judgements)  |  Perfect ±{:.0} ms",
                chart.metadata.title,
                chart.metadata.difficulty_name,
                chart.notes.len(),
                chart.total_judgements(),
                windows.perfect_ms
            ),
            20.0,
            30.0,
            26.0,
            WHITE,
        );

        next_frame().await;
    }
}
