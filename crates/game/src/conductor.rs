//! Conductor: the game's single source of time.
//!
//! Principle #1 of the project: audio drives time. Nothing in the game is
//! positioned with frame deltas; everything asks the conductor for the song
//! position instead. The position comes from the audio backend (kira) and is
//! smoothed by `rhythm_core::SmoothClock`, because the backend only updates it
//! once per audio buffer.

// TODO(M1-2): remove this once main.rs uses the conductor.
#![allow(dead_code)]

use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
use kira::sound::PlaybackState;
use kira::{AudioManager, AudioManagerSettings, DefaultBackend};
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

    pub fn set_calibration_ms(&mut self, ms: f64) {
        self.calibration_ms = ms;
    }

    pub fn is_playing(&self) -> bool {
        self.handle
            .as_ref()
            .is_some_and(|h| h.state() == PlaybackState::Playing)
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
