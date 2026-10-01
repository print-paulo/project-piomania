//! Judgement windows, inspired by osu!mania.
//!
//! Convention: `delta_ms = input_time - note_time`.
//! Negative = early, positive = late.

/// Result of a judgement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Judgement {
    Perfect,
    Great,
    Good,
    Miss,
}

/// Hit windows in milliseconds (half-width: applies to both early and late hits).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JudgementWindows {
    pub perfect_ms: f64,
    pub great_ms: f64,
    pub good_ms: f64,
    /// Beyond `good_ms` and up to this value, pressing too early counts as a Miss.
    /// Earlier than that, the input is ignored (the note is still far away).
    pub miss_ms: f64,
    /// Window multiplier when releasing a hold (releases are usually more lenient).
    pub release_multiplier: f64,
}

impl JudgementWindows {
    /// Values based on the osu!mania formula as a function of OD (Overall Difficulty).
    /// TODO: double-check these numbers against the osu! wiki and tune them in playtests.
    pub fn from_od(od: f64) -> Self {
        Self {
            perfect_ms: 64.0 - 3.0 * od,
            great_ms: 97.0 - 3.0 * od,
            good_ms: 127.0 - 3.0 * od,
            miss_ms: 188.0 - 3.0 * od,
            release_multiplier: 1.5,
        }
    }

    /// Judges a press on a note (or on a hold head).
    /// Returns `None` if it is still too early and the input should be ignored.
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

    /// Judges the release of a hold (always returns a judgement).
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

    /// `true` when the note is so late that it becomes an automatic Miss.
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
        // 50 ms would be Great on a press, but on a release it is still Perfect (40 * 1.5 = 60).
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
