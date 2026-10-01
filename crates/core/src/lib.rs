//! Game core.
//!
//! Golden rule: this crate knows NOTHING about rendering, windows or audio.
//! It only receives times (in ms) and returns judgements and scores.
//! That keeps everything testable and lets us swap Macroquad for wgpu later.

pub mod judgement;
pub mod score;

pub use judgement::{Judgement, JudgementWindows};
pub use score::ScoreState;