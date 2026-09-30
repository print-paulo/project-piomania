//! Pontuação de 0 a 1.000.000, com foco maior em precisão que em combo.
//!
//! - 700.000 pontos vêm da precisão (peso de cada julgamento).
//! - 300.000 pontos vêm do combo.
//!
//! Cada "objeto julgado" conta uma vez: nota simples = 1; hold = 2 (cabeça + soltura).
//! Modificadores (Hidden, DT...) que passam de 1.000.000 entram numa etapa futura.

use crate::judgement::Judgement;

pub const MAX_SCORE: u32 = 1_000_000;
pub const ACCURACY_SHARE: f64 = 700_000.0;
pub const COMBO_SHARE: f64 = 300_000.0;
/// Combo acima disso não aumenta mais o valor de cada acerto.
pub const COMBO_CAP: u32 = 100;

/// Peso de cada julgamento na precisão.
pub fn accuracy_weight(j: Judgement) -> f64 {
    match j {
        Judgement::Perfect => 1.0,
        Judgement::Great => 2.0 / 3.0,
        Judgement::Good => 1.0 / 3.0,
        Judgement::Miss => 0.0,
    }
}

#[derive(Debug, Clone)]
pub struct ScoreState {
    total_objects: u32,
    judged: u32,
    accuracy_points: f64,
    combo: u32,
    max_combo: u32,
    combo_points: f64,
    ideal_combo_points: f64,
    counts: [u32; 4],
}

impl ScoreState {
    /// `total_objects` = notas + 2 × holds.
    pub fn new(total_objects: u32) -> Self {
        let ideal_combo_points = (1..=total_objects).map(|i| i.min(COMBO_CAP) as f64).sum();
        Self {
            total_objects,
            judged: 0,
            accuracy_points: 0.0,
            combo: 0,
            max_combo: 0,
            combo_points: 0.0,
            ideal_combo_points,
            counts: [0; 4],
        }
    }

    pub fn register(&mut self, j: Judgement) {
        self.judged += 1;
        self.counts[j as usize] += 1;
        self.accuracy_points += accuracy_weight(j);
        if j == Judgement::Miss {
            self.combo = 0;
        } else {
            self.combo += 1;
            self.max_combo = self.max_combo.max(self.combo);
        }
        self.combo_points += self.combo.min(COMBO_CAP) as f64;
    }

    /// Pontuação atual (0..=1.000.000), já considerando o total da música.
    pub fn score(&self) -> u32 {
        if self.total_objects == 0 {
            return 0;
        }
        let acc = ACCURACY_SHARE * self.accuracy_points / self.total_objects as f64;
        let combo = COMBO_SHARE * self.combo_points / self.ideal_combo_points;
        ((acc + combo).round() as u32).min(MAX_SCORE)
    }

    /// Precisão em % sobre o que já foi julgado.
    pub fn accuracy_percent(&self) -> f64 {
        if self.judged == 0 {
            return 100.0;
        }
        100.0 * self.accuracy_points / self.judged as f64
    }

    pub fn combo(&self) -> u32 {
        self.combo
    }

    pub fn max_combo(&self) -> u32 {
        self.max_combo
    }

    pub fn count(&self, j: Judgement) -> u32 {
        self.counts[j as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_perfect_is_one_million() {
        let mut s = ScoreState::new(250);
        for _ in 0..250 {
            s.register(Judgement::Perfect);
        }
        assert_eq!(s.score(), MAX_SCORE);
        assert_eq!(s.max_combo(), 250);
    }

    #[test]
    fn all_miss_is_zero() {
        let mut s = ScoreState::new(50);
        for _ in 0..50 {
            s.register(Judgement::Miss);
        }
        assert_eq!(s.score(), 0);
    }

    #[test]
    fn a_miss_costs_more_than_a_good() {
        let run = |mid: Judgement| {
            let mut s = ScoreState::new(200);
            for i in 0..200 {
                s.register(if i == 100 { mid } else { Judgement::Perfect });
            }
            s.score()
        };
        assert!(run(Judgement::Good) > run(Judgement::Miss));
        assert!(run(Judgement::Good) < MAX_SCORE);
    }

    #[test]
    fn empty_chart_scores_zero() {
        assert_eq!(ScoreState::new(0).score(), 0);
    }
}