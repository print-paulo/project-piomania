//! Scroll timeline: converts song time into scroll distance.
//!
//! A scroll point says "from `time_ms` onward, notes move at `speed`".
//! The distance is the area under the speed graph, summed segment by segment:
//! speed × duration for each stretch where that speed applies.

/// From `time_ms` onward, notes scroll at `speed` (1.0 = normal speed).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollPoint {
    pub time_ms: f64,
    pub speed: f64,
}

#[derive(Debug, Clone)]
pub struct ScrollTimeline {
    points: Vec<ScrollPoint>,
}

impl ScrollTimeline {
    pub fn new(mut points: Vec<ScrollPoint>) -> Self {
        points.sort_by(|a, b| a.time_ms.total_cmp(&b.time_ms));
        Self { points }
    }

    /// Total scroll distance from time 0 up to `time_ms`.
    pub fn distance_at(&self, time_ms: f64) -> f64 {
        if time_ms <= 0.0 {
            return 0.0;
        }

        self.points
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let start = point.time_ms.max(0.0);
                let end = self
                    .points
                    .get(index + 1)
                    .map_or(time_ms, |next| next.time_ms.min(time_ms));
                (end - start).max(0.0) * point.speed
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compares floats with a tiny tolerance (never use `==` on f64 results).
    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn timeline(points: &[(f64, f64)]) -> ScrollTimeline {
        ScrollTimeline::new(
            points
                .iter()
                .map(|&(time_ms, speed)| ScrollPoint { time_ms, speed })
                .collect(),
        )
    }

    #[test]
    fn constant_speed_one() {
        let t = timeline(&[(0.0, 1.0)]);
        assert!(approx_eq(t.distance_at(1000.0), 1000.0));
    }

    #[test]
    fn constant_speed_two() {
        let t = timeline(&[(0.0, 2.0)]);
        assert!(approx_eq(t.distance_at(500.0), 1000.0));
    }

    #[test]
    fn speed_change_midway() {
        // 1.0x until 1000 ms, then 2.0x: 1000 + 500 * 2 = 2000 at 1500 ms.
        let t = timeline(&[(0.0, 1.0), (1000.0, 2.0)]);
        assert!(approx_eq(t.distance_at(1500.0), 2000.0));
    }

    // TODO (yours): add tests for exactly on a change point, before the first
    // point, an empty list, and unsorted input.
}
