//! Chart format for the game (JSON, version 1).
//!
//! Everything here is data: the game reads the chart and plays it back. No gameplay rules live here.
//! See `docs/chart-format.md` for a description of each field.

use serde::{Deserialize, Serialize};
use std::fmt;

pub const FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub format_version: u32,
    pub metadata: Metadata,
    pub audio: Audio,
    /// BPM changes (at least one, at time 0).
    pub timing: Vec<TimingPoint>,
    /// Scroll speed changes (optional).
    #[serde(default)]
    pub scroll: Vec<ScrollChange>,
    pub judgement_lines: Vec<JudgementLine>,
    pub lanes: Vec<Lane>,
    pub notes: Vec<Note>,
    #[serde(default)]
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub artist: String,
    pub charter: String,
    pub difficulty_name: String,
    /// Number of lanes (4 or 7).
    pub keys: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    pub file: String,
    #[serde(default)]
    pub offset_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingPoint {
    pub time_ms: f64,
    pub bpm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollChange {
    pub time_ms: f64,
    pub speed: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Easing {
    #[default]
    Linear,
    In,
    Out,
    InOut,
    /// Jumps straight to the keyframe value (no interpolation).
    Step,
}

fn one() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineKeyframe {
    pub time_ms: f64,
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    pub rotation_deg: f32,
    #[serde(default = "one")]
    pub alpha: f32,
    #[serde(default)]
    pub easing: Easing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgementLine {
    pub id: String,
    pub keyframes: Vec<LineKeyframe>,
}

/// Lane shape at a given moment: a Bézier curve (2 points = straight, 3 = quadratic, 4 = cubic)
/// in the judgement line's local space. The last point is where the note is judged.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathKeyframe {
    pub time_ms: f64,
    pub points: Vec<[f32; 2]>,
    #[serde(default)]
    pub easing: Easing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlphaKeyframe {
    pub time_ms: f64,
    pub alpha: f32,
    #[serde(default)]
    pub easing: Easing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lane {
    pub index: u8,
    /// `id` of the judgement line this lane belongs to.
    pub line: String,
    pub path: Vec<PathKeyframe>,
    #[serde(default)]
    pub alpha: Vec<AlphaKeyframe>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub time_ms: f64,
    pub lane: u8,
    /// Only present on holds: the time at which the hold ends.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time_ms: Option<f64>,
}

impl Note {
    pub fn is_hold(&self) -> bool {
        self.end_time_ms.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub time_ms: f64,
    #[serde(flatten)]
    pub kind: EventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventKind {
    Flash {
        duration_ms: f64,
        color: [f32; 3],
    },
    Shake {
        duration_ms: f64,
        intensity: f32,
    },
    Palette {
        duration_ms: f64,
        background: [f32; 3],
    },
}

#[derive(Debug)]
pub enum ChartError {
    Parse(serde_json::Error),
    Invalid(String),
}

impl fmt::Display for ChartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChartError::Parse(e) => write!(f, "invalid JSON: {e}"),
            ChartError::Invalid(msg) => write!(f, "invalid chart: {msg}"),
        }
    }
}

impl std::error::Error for ChartError {}

impl Chart {
    /// Parses and validates a chart from JSON text.
    pub fn from_json(text: &str) -> Result<Chart, ChartError> {
        let chart: Chart = serde_json::from_str(text).map_err(ChartError::Parse)?;
        chart.validate()?;
        Ok(chart)
    }

    /// Number of judged objects: regular notes count as 1, holds count as 2 (head + release).
    pub fn total_judgements(&self) -> u32 {
        self.notes
            .iter()
            .map(|n| if n.is_hold() { 2 } else { 1 })
            .sum()
    }

    pub fn validate(&self) -> Result<(), ChartError> {
        let bad = |m: String| Err(ChartError::Invalid(m));

        if self.format_version != FORMAT_VERSION {
            return bad(format!(
                "format_version {} is not supported (expected {FORMAT_VERSION})",
                self.format_version
            ));
        }
        if self.timing.is_empty() {
            return bad("`timing` must have at least one point".into());
        }
        if self.timing.iter().any(|t| t.bpm <= 0.0) {
            return bad("every BPM must be greater than zero".into());
        }
        if self.judgement_lines.is_empty() {
            return bad("`judgement_lines` is empty".into());
        }
        for line in &self.judgement_lines {
            if line.keyframes.is_empty() {
                return bad(format!("judgement line `{}` has no keyframes", line.id));
            }
        }
        if self.lanes.len() != self.metadata.keys as usize {
            return bad(format!(
                "metadata.keys = {}, but there are {} lanes",
                self.metadata.keys,
                self.lanes.len()
            ));
        }
        for lane in &self.lanes {
            if !self.judgement_lines.iter().any(|l| l.id == lane.line) {
                return bad(format!(
                    "lane {} points to line `{}`, which does not exist",
                    lane.index, lane.line
                ));
            }
            if lane.path.is_empty() {
                return bad(format!("lane {} has no path", lane.index));
            }
            if lane.path.iter().any(|k| k.points.len() < 2) {
                return bad(format!(
                    "lane {}: each path needs at least 2 points",
                    lane.index
                ));
            }
        }
        for (i, n) in self.notes.iter().enumerate() {
            if n.lane >= self.metadata.keys {
                return bad(format!("note #{i}: lane {} is out of range", n.lane));
            }
            if let Some(end) = n.end_time_ms {
                if end <= n.time_ms {
                    return bad(format!("note #{i}: hold ends before it starts"));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../charts/example/chart.json");

    #[test]
    fn example_chart_parses_and_validates() {
        let chart = Chart::from_json(EXAMPLE).expect("the example chart must be valid");
        assert_eq!(chart.metadata.keys, 4);
        assert_eq!(chart.lanes.len(), 4);
        assert!(chart.notes.iter().any(|n| n.is_hold()));
        assert!(chart.total_judgements() > chart.notes.len() as u32);
    }

    #[test]
    fn rejects_note_in_missing_lane() {
        let mut chart: Chart = serde_json::from_str(EXAMPLE).unwrap();
        chart.notes.push(Note {
            time_ms: 100.0,
            lane: 9,
            end_time_ms: None,
        });
        assert!(chart.validate().is_err());
    }

    #[test]
    fn rejects_hold_that_ends_before_start() {
        let mut chart: Chart = serde_json::from_str(EXAMPLE).unwrap();
        chart.notes.push(Note {
            time_ms: 500.0,
            lane: 0,
            end_time_ms: Some(400.0),
        });
        assert!(chart.validate().is_err());
    }
}
