//! Conductor: the game's single source of time.
//!
//! Principle #1 of the project: audio drives time. Nothing in the game is
//! positioned with frame deltas; everything asks the conductor for the song
//! position instead. The position comes from the audio backend (kira) and is
//! smoothed by `rhythm_core::SmoothClock`, because the backend only updates it
//! once per audio buffer.

use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
use kira::sound::PlaybackState;
use kira::{AudioManager, AudioManagerSettings, DefaultBackend, Tween};
use rhythm_core::SmoothClock;
use std::error::Error;
use std::time::Instant;

pub struct Conductor {
    manager: AudioManager<DefaultBackend>,
    sound: StaticSoundData,
    handle: Option<StaticSoundHandle>,
    clock: SmoothClock,
    /// Origin of the real-time clock fed to `SmoothClock`.
    wall_origin: Instant,
    /// Chart `audio.offset_ms`: positive means the chart starts later than the audio.
    offset_ms: f64,
    /// Player calibration (M1-9). Positive = audio is heard late.
    calibration_ms: f64,
}

impl Conductor {
    /// Loads the audio file (fully decoded in memory) without starting it.
    pub fn new(audio_path: &str, offset_ms: f64) -> Result<Self, Box<dyn Error>> {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
        let sound = StaticSoundData::from_file(audio_path)?;
        Ok(Self {
            manager,
            sound,
            handle: None,
            clock: SmoothClock::default(),
            wall_origin: Instant::now(),
            offset_ms,
            calibration_ms: 0.0,
        })
    }

    /// Starts the song from the beginning.
    pub fn play(&mut self) -> Result<(), Box<dyn Error>> {
        self.clock.reset();
        self.handle = Some(self.manager.play(self.sound.clone())?);
        Ok(())
    }

    // `#[allow(dead_code)]` because main.rs does not call this until M1-9.
    #[allow(dead_code)] // TODO(M1-9)
    pub fn set_calibration_ms(&mut self, ms: f64) {
        self.calibration_ms = ms;
    }

    // `#[allow(dead_code)]` because main.rs uses `playback_state()` instead.
    #[allow(dead_code)]
    pub fn is_playing(&self) -> bool {
        self.handle
            .as_ref()
            .is_some_and(|h| h.state() == PlaybackState::Playing)
    }

    /// Pauses the song if it is currently playing.
    ///
    /// `if let Some(h) = &mut self.handle` gives us a mutable reference to the
    /// handle only if it exists — `Option` acts as a guard here.
    /// `&mut self` is needed because `pause()` mutates kira's internal state.
    pub fn pause(&mut self) {
        if let Some(h) = &mut self.handle {
            if h.state() == PlaybackState::Playing {
                // `pause` returns `()`, not `Result` — kira 0.10 API.
                h.pause(Tween::default());
            }
        }
    }

    /// Resumes the song if it is currently paused.
    pub fn resume(&mut self) {
        if let Some(h) = &mut self.handle {
            if h.state() == PlaybackState::Paused {
                h.resume(Tween::default());
            }
        }
    }

    /// Toggles between playing and paused; ignores other states (e.g. Stopping).
    pub fn toggle_pause(&mut self) {
        // Read the state first (immutable borrow), then call the right method.
        let state = self.handle.as_ref().map(|h| h.state());
        match state {
            Some(PlaybackState::Playing) => self.pause(),
            Some(PlaybackState::Paused) => self.resume(),
            _ => {}
        }
    }

    /// Raw audio position in ms (straight from kira, no smoothing or offsets).
    ///
    /// This advances in steps (~5–20 ms) because kira only updates once per
    /// audio buffer. Use `time_ms()` for game logic; use this for debugging.
    /// Returns 0.0 when no handle exists (before `play()` is called).
    pub fn raw_audio_ms(&self) -> f64 {
        // `&self` (not `&mut self`) because we only read; no clock update needed.
        self.handle.as_ref().map_or(0.0, |h| h.position() * 1000.0)
    }

    /// Current playback state from kira. Returns `Stopped` when no handle exists.
    pub fn playback_state(&self) -> PlaybackState {
        self.handle
            .as_ref()
            .map_or(PlaybackState::Stopped, |h| h.state())
    }

    /// Current song time in ms, in the chart's timeline (what notes use).
    /// Call it once per frame and use the same value for the whole frame.
    /// Before `play()` it returns 0.
    pub fn time_ms(&mut self) -> f64 {
        let Some(handle) = &self.handle else {
            return 0.0;
        };
        let audio_ms = handle.position() * 1000.0;
        let playing = handle.state() == PlaybackState::Playing;
        let wall_ms = self.wall_origin.elapsed().as_secs_f64() * 1000.0;
        let smoothed = self.clock.update(audio_ms, wall_ms, playing);
        smoothed - self.offset_ms - self.calibration_ms
    }
}
