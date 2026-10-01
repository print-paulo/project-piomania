//! Smoothing of the song clock.
//!
//! Audio backends report the playback position in chunks (one update per audio
//! buffer, typically every 5-20 ms), so reading it once per frame gives a
//! "staircase" instead of a smooth line. `SmoothClock` fills the gaps by
//! advancing with a real-time clock between audio updates. The estimate never
//! runs more than `max_drift_ms` ahead of the last audio position: if the audio
//! stalls, the clock holds there instead of jumping backwards.
//!
//! This crate knows nothing about audio: callers feed it plain numbers (ms).

/// Maximum amount (ms) the estimate may run ahead of the last audio position.
pub const DEFAULT_MAX_DRIFT_MS: f64 = 30.0;

#[derive(Debug, Clone)]
pub struct SmoothClock {
    max_drift_ms: f64,
    /// Last audio position seen (ms).
    last_audio_ms: f64,
    /// Wall-clock time (ms) when `last_audio_ms` last changed.
    last_change_wall_ms: f64,
    started: bool,
}

impl SmoothClock {
    pub fn new(max_drift_ms: f64) -> Self {
        Self {
            max_drift_ms,
            last_audio_ms: 0.0,
            last_change_wall_ms: 0.0,
            started: false,
        }
    }

    /// Forget everything (call when the song restarts or seeks).
    pub fn reset(&mut self) {
        self.started = false;
    }

    /// `audio_ms`: position reported by the audio backend.
    /// `wall_ms`: a monotonic real-time clock (any origin, same origin every call).
    /// `playing`: `false` freezes the estimate on the audio position (paused/stopped).
    /// Returns the smoothed song position in ms.
    pub fn update(&mut self, audio_ms: f64, wall_ms: f64, playing: bool) -> f64 {
        if !self.started || audio_ms != self.last_audio_ms {
            self.last_audio_ms = audio_ms;
            self.last_change_wall_ms = wall_ms;
            self.started = true;
        }
        if !playing {
            return audio_ms;
        }
        let estimate = self.last_audio_ms + (wall_ms - self.last_change_wall_ms);
        // Hold at the limit instead of snapping back, so time never goes backwards
        // just because the audio backend updates less often than expected.
        estimate.min(audio_ms + self.max_drift_ms)
    }
}

impl Default for SmoothClock {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_DRIFT_MS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_call_returns_audio_position() {
        let mut c = SmoothClock::default();
        assert_eq!(c.update(100.0, 5000.0, true), 100.0);
    }

    #[test]
    fn interpolates_between_audio_updates() {
        let mut c = SmoothClock::default();
        c.update(100.0, 1000.0, true);
        // Audio has not moved, but 8 ms of real time passed.
        assert_eq!(c.update(100.0, 1008.0, true), 108.0);
    }

    #[test]
    fn holds_at_max_drift_when_audio_stalls() {
        let mut c = SmoothClock::default();
        c.update(100.0, 1000.0, true);
        // 200 ms of wall time with a frozen audio position: hold at audio + max drift.
        assert_eq!(c.update(100.0, 1200.0, true), 130.0);
    }

    #[test]
    fn never_goes_backwards_with_slow_audio_updates() {
        // Audio position only advances every 40 ms; frames come at ~60 fps.
        let mut c = SmoothClock::default();
        let mut prev = f64::MIN;
        let mut wall = 0.0_f64;
        while wall < 2000.0 {
            let audio = (wall / 40.0).floor() * 40.0;
            let t = c.update(audio, wall, true);
            assert!(t >= prev, "time went backwards: {prev} -> {t}");
            prev = t;
            wall += 1000.0 / 60.0;
        }
    }

    #[test]
    fn follows_new_audio_positions() {
        let mut c = SmoothClock::default();
        c.update(100.0, 1000.0, true);
        assert_eq!(c.update(116.0, 1016.0, true), 116.0);
        assert_eq!(c.update(116.0, 1020.0, true), 120.0);
    }

    #[test]
    fn paused_clock_does_not_advance() {
        let mut c = SmoothClock::default();
        c.update(100.0, 1000.0, true);
        assert_eq!(c.update(100.0, 1010.0, false), 100.0);
    }
}
