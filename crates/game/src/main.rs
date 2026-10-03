//! Entry point. For now: opens the window, loads the example chart
//! and draws the judgement line + straight lanes (just to validate the setup).

mod conductor;

use conductor::Conductor;
use macroquad::prelude::*;
use rhythm_chart::Chart;
use rhythm_core::JudgementWindows;
use std::path::Path;

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
    let chart_path = "charts/example/chart.json";
    let text = std::fs::read_to_string(chart_path)
        .expect("run the game from the repository root (cargo run -p piomania)");
    let chart = Chart::from_json(&text).expect("invalid chart");
    let windows = JudgementWindows::default();

    // Resolve the audio path relative to the chart file so the game works
    // regardless of cwd, as long as chart_path itself is correct.
    let audio_path = Path::new(chart_path)
        .parent()
        .expect("chart path has no parent directory")
        .join(&chart.audio.file);

    let mut conductor = Conductor::new(
        audio_path.to_str().expect("audio path is not valid UTF-8"),
        chart.audio.offset_ms,
    )
    .expect("failed to initialise audio (Conductor::new)");

    conductor.play().expect("failed to start audio playback");

    loop {
        // --- Sample conductor once at the start of the frame (design-intent §conductor) ---
        let song_time_ms = conductor.time_ms();
        let raw_audio_ms = conductor.raw_audio_ms();
        let playback_state = conductor.playback_state();

        // Space toggles pause / resume.
        if is_key_pressed(KeyCode::Space) {
            conductor.toggle_pause();
        }

        // --- Rendering ---
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

        // Header line (existing).
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

        // --- Debug HUD (M1-2) ---
        // Smooth time formatted as MM:SS.mmm plus its raw ms value.
        let minutes = ((song_time_ms.max(0.0) / 1000.0) / 60.0).floor() as u32;
        let seconds = (song_time_ms.max(0.0) / 1000.0) % 60.0;

        draw_text(
            format!(
                "Time: {:02}:{:06.3} ({:.1} ms)",
                minutes, seconds, song_time_ms
            ),
            20.0,
            60.0,
            22.0,
            YELLOW,
        );
        draw_text(
            // `playback_state` is printed with {:?} (Debug); we never name its type
            // in this file, so no import of PlaybackState is needed (avoiding an
            // unused-import clippy warning under -D warnings).
            format!(
                "Raw Audio: {:.1} ms  |  State: {:?}",
                raw_audio_ms, playback_state
            ),
            20.0,
            85.0,
            22.0,
            YELLOW,
        );
        draw_text("[Space] Pause / Resume", 20.0, 110.0, 22.0, GRAY);

        next_frame().await;
    }
}
