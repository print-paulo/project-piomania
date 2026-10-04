//! Entry point. Loads the example chart and renders:
//!   - the judgement line
//!   - 4 straight lane guides
//!   - notes falling toward the judgement line (M1-4)

mod conductor;

use conductor::Conductor;
use macroquad::prelude::*;
use rhythm_chart::Chart;
use rhythm_core::{JudgementWindows, ScrollPoint, ScrollTimeline};
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

    // Build the scroll timeline from the chart's scroll-speed changes.
    // `ScrollChange` and `ScrollPoint` carry the same data; we just bridge the types.
    let scroll_points: Vec<ScrollPoint> = chart
        .scroll
        .iter()
        .map(|s| ScrollPoint {
            time_ms: s.time_ms,
            speed: s.speed,
        })
        .collect();
    let scroll_timeline = ScrollTimeline::new(scroll_points);

    // How many scroll-distance units fit the full visible lane length
    // (spawn → judge point).  At speed 1.0 this equals milliseconds of notes
    // shown at once.  Placeholder value; a per-player speed setting comes later.
    const VISIBLE_DISTANCE: f64 = 800.0;

    // Note rectangle dimensions in screen-height units.
    const NOTE_WIDTH: f32 = 0.06;
    const NOTE_HEIGHT: f32 = 0.018;

    loop {
        // Sample conductor once per frame (design-intent §conductor).
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

        // Cumulative scroll distance reached at this frame.
        let current_dist = scroll_timeline.distance_at(song_time_ms);

        for lane in &chart.lanes {
            let p = &lane.path[0].points;
            // spawn = top of the lane (where notes enter), judge = bottom (where they're hit).
            let spawn = p[0];
            let judge = p[p.len() - 1];

            let spawn_sx = lx + spawn[0] * unit;
            let spawn_sy = ly + spawn[1] * unit;
            let judge_sx = lx + judge[0] * unit;
            let judge_sy = ly + judge[1] * unit;

            draw_line(spawn_sx, spawn_sy, judge_sx, judge_sy, 2.0, DARKGRAY);

            // Render each note that belongs to this lane.
            for note in &chart.notes {
                if note.lane != lane.index {
                    continue;
                }

                let note_dist = scroll_timeline.distance_at(note.time_ms);
                // Positive delta: note is still ahead of the current position.
                let delta = note_dist - current_dist;

                // Cull notes outside the visible window.
                if delta < 0.0 || delta > VISIBLE_DISTANCE {
                    continue;
                }

                // t = 0 → note is at the judge point; t = 1 → note is at spawn.
                let t = (delta / VISIBLE_DISTANCE) as f32;

                // Linear interpolation along the straight lane.
                let nx = judge_sx + t * (spawn_sx - judge_sx);
                let ny = judge_sy + t * (spawn_sy - judge_sy);

                // Outer lanes (0, 3) are blue; inner lanes (1, 2) are white —
                // matching common 4K colour schemes.
                let color = if note.lane % 4 == 0 || note.lane % 4 == 3 {
                    Color::new(0.35, 0.70, 1.00, 1.0)
                } else {
                    Color::new(1.00, 1.00, 1.00, 1.0)
                };

                let hw = NOTE_WIDTH * unit / 2.0;
                let hh = NOTE_HEIGHT * unit / 2.0;
                draw_rectangle(nx - hw, ny - hh, hw * 2.0, hh * 2.0, color);
            }
        }

        // Judgement line drawn on top of lanes and notes.
        draw_line(lx - 0.3 * unit, ly, lx + 0.3 * unit, ly, 4.0, WHITE);

        // Header.
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
            // `playback_state` is printed with {:?} (Debug) — no explicit import needed.
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
