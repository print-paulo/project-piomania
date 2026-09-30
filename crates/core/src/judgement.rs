//! Janelas de julgamento, inspiradas no osu!mania.
//!
//! Convenção: `delta_ms = tempo_do_input - tempo_da_nota`.
//! Negativo = cedo, positivo = tarde.

/// Resultado de um julgamento.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Judgement {
    Perfect,
    Great,
    Good,
    Miss,
}

/// Janelas de acerto em milissegundos (metade da janela: vale para cedo e tarde).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JudgementWindows {
    pub perfect_ms: f64,
    pub great_ms: f64,
    pub good_ms: f64,
    /// Além de `good_ms` e até aqui, apertar cedo demais conta como Miss.
    /// Mais cedo que isso, o input é ignorado (nota ainda longe).
    pub miss_ms: f64,
    /// Multiplicador das janelas ao soltar um hold (a soltura costuma ser mais permissiva).
    pub release_multiplier: f64,
}

impl JudgementWindows {
    /// Valores baseados na fórmula do osu!mania em função do OD (Overall Difficulty).
    /// TODO: conferir os números com a wiki do osu! e ajustar nos playtests.
    pub fn from_od(od: f64) -> Self {
        Self {
            perfect_ms: 64.0 - 3.0 * od,
            great_ms: 97.0 - 3.0 * od,
            good_ms: 127.0 - 3.0 * od,
            miss_ms: 188.0 - 3.0 * od,
            release_multiplier: 1.5,
        }
    }

    /// Julga o toque numa nota (ou na cabeça de um hold).
    /// Retorna `None` se ainda está cedo demais e o input deve ser ignorado.
    pub fn judge_press(&self, delta_ms: f64) -> Option<Judgement> {
        let d = delta_ms.abs();
        if d <= self.perfect_ms {
            Some(Judgement::Perfect)
        } else if d <= self.great_ms {
            Some(Judgement::Great)
        } else if d <= self.good_ms {
            Some(Judgement::Good)
        } else if delta_ms < 0.0 && d <= self.miss_ms {
            Some(Judgement::Miss)
        } else {
            None
        }
    }

    /// Julga a soltura de um hold (sempre devolve um julgamento).
    pub fn judge_release(&self, delta_ms: f64) -> Judgement {
        let d = delta_ms.abs();
        let m = self.release_multiplier;
        if d <= self.perfect_ms * m {
            Judgement::Perfect
        } else if d <= self.great_ms * m {
            Judgement::Great
        } else if d <= self.good_ms * m {
            Judgement::Good
        } else {
            Judgement::Miss
        }
    }

    /// `true` quando a nota passou tanto do tempo que vira Miss automático.
    pub fn is_expired(&self, delta_ms: f64) -> bool {
        delta_ms > self.good_ms
    }
}

impl Default for JudgementWindows {
    fn default() -> Self {
        Self::from_od(8.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn od8_windows() {
        let w = JudgementWindows::from_od(8.0);
        assert_eq!(w.perfect_ms, 40.0);
        assert_eq!(w.great_ms, 73.0);
        assert_eq!(w.good_ms, 103.0);
        assert_eq!(w.miss_ms, 164.0);
    }

    #[test]
    fn press_classification() {
        let w = JudgementWindows::default();
        assert_eq!(w.judge_press(0.0), Some(Judgement::Perfect));
        assert_eq!(w.judge_press(-40.0), Some(Judgement::Perfect));
        assert_eq!(w.judge_press(50.0), Some(Judgement::Great));
        assert_eq!(w.judge_press(-100.0), Some(Judgement::Good));
        assert_eq!(w.judge_press(-150.0), Some(Judgement::Miss));
        assert_eq!(w.judge_press(-300.0), None);
    }

    #[test]
    fn release_is_more_lenient() {
        let w = JudgementWindows::default();
        // 50 ms seria Great num toque, mas na soltura ainda é Perfect (40 * 1.5 = 60).
        assert_eq!(w.judge_release(50.0), Judgement::Perfect);
        assert_eq!(w.judge_release(400.0), Judgement::Miss);
    }

    #[test]
    fn expiry() {
        let w = JudgementWindows::default();
        assert!(!w.is_expired(100.0));
        assert!(w.is_expired(104.0));
    }
}
