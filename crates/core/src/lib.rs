//! Game core.
//!
//! Golden rule: this crate knows NOTHING about rendering, windows or audio.
//! It only receives times (in ms) and returns judgements and scores.
//! That keeps everything testable and lets us swap Macroquad for wgpu later.

pub mod clock;
pub mod judgement;
pub mod score;
pub mod timeline;

pub use clock::SmoothClock;
pub use judgement::{Judgement, JudgementWindows};
pub use score::ScoreState;
