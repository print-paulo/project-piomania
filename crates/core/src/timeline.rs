//! Scroll timeline: converts song time into scroll distance.
//!
//! A scroll point says "from `time_ms` onward, notes move at `speed`".
//! The distance is the area under the speed graph, summed segment by segment:
//! speed × duration for each stretch where that speed applies.

/// Speed used before the first scroll point (and for an empty timeline).
const DEFAULT_SPEED: f64 = 1.0;

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
    /// Sorts the points by time. If nothing is defined at time 0 (empty list, or
    /// the first point comes later), an implicit point with `DEFAULT_SPEED` is
    /// added there, so notes never freeze. For points with the same time, the
    /// last one wins.
    pub fn new(mut points: Vec<ScrollPoint>) -> Self {
        points.sort_by(|a, b| a.time_ms.total_cmp(&b.time_ms));

        let starts_at_zero = points.first().is_some_and(|first| first.time_ms <= 0.0);
        if !starts_at_zero {
            points.insert(
                0,
                ScrollPoint {
                    time_ms: 0.0,
                    speed: DEFAULT_SPEED,
                },
            );
        }

        Self { points }
    }

    /// Total scroll distance from time 0 up to `time_ms`.
    pub fn distance_at(&self, time_ms: f64) -> f64 {
        // TODO: negative times (audio offset, count-in) return 0 for now, so
        // notes stand still before the song starts. Later: extend backwards
        // using the first point's speed (negative distances).
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

    #[test]
    fn empty_list_uses_default_speed() {
        let t = timeline(&[]);
        assert!(approx_eq(t.distance_at(1000.0), 1000.0));
    }

    #[test]
    fn default_speed_before_first_point() {
        // Nothing defined before 1000 ms: default 1.0x, then 2.0x.
        let t = timeline(&[(1000.0, 2.0)]);
        assert!(approx_eq(t.distance_at(500.0), 500.0));
        // 1000 at 1.0x + 500 at 2.0x = 2000.
        assert!(approx_eq(t.distance_at(1500.0), 2000.0));
    }

    #[test]
    fn exactly_on_a_change_point() {
        let t = timeline(&[(0.0, 1.0), (1000.0, 2.0)]);
        assert!(approx_eq(t.distance_at(1000.0), 1000.0));
    }

    #[test]
    fn unsorted_input_gives_same_result_as_sorted() {
        let sorted = timeline(&[(0.0, 1.0), (1000.0, 2.0)]);
        let unsorted = timeline(&[(1000.0, 2.0), (0.0, 1.0)]);
        assert!(approx_eq(
            unsorted.distance_at(1500.0),
            sorted.distance_at(1500.0)
        ));
    }

    #[test]
    fn same_time_last_point_wins() {
        let t = timeline(&[(0.0, 1.0), (0.0, 3.0)]);
        assert!(approx_eq(t.distance_at(100.0), 300.0));
    }

    #[test]
    fn negative_time_has_no_distance_yet() {
        let t = timeline(&[(0.0, 1.0)]);
        assert!(approx_eq(t.distance_at(-200.0), 0.0));
        assert!(approx_eq(t.distance_at(0.0), 0.0));
    }

    #[test]
    fn negative_speed_moves_notes_back() {
        // 1.0x for 1000 ms, then -1.0x for 500 ms: 1000 - 500 = 500.
        let t = timeline(&[(0.0, 1.0), (1000.0, -1.0)]);
        assert!(approx_eq(t.distance_at(1500.0), 500.0));
    }
}
