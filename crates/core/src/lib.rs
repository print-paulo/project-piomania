//! Núcleo do jogo.
//!
//! Regra de ouro: este crate NÃO conhece render, janela nem áudio.
//! Ele só recebe tempos (em ms) e devolve julgamentos e pontuação.
//! Isso deixa tudo testável e permite trocar Macroquad por wgpu depois.

pub mod judgement;
pub mod score;

pub use judgement::{Judgement, JudgementWindows};
pub use score::ScoreState;