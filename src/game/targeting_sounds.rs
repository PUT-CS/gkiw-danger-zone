use crate::audio::{audio::Audio, audio_manager::SoundEffect, sound::SoundID};

pub struct TargetingSounds {
    pub current: Option<SoundID>,
}

impl TargetingSounds {
    pub fn new() -> Self {
        Self { current: None }
    }

    pub fn stop(&mut self, audio: &Audio) {
        if let Some(id) = self.current {
            audio.stop(id);
            self.current = None
        }
    }

    pub fn play(&mut self, effect: SoundEffect, audio: &Audio) {
        self.stop(audio);
        match effect {
            SoundEffect::Seeking => {
                let id = audio.play(SoundEffect::Seeking, true);
                self.current = Some(id);
            }
            SoundEffect::Locking => {
                let id = audio.play(SoundEffect::Locking, true);
                self.current = Some(id);
            }
            SoundEffect::Locked => {
                let id = audio.play(SoundEffect::Locked, true);
                self.current = Some(id);
            }
            _ => panic!(),
        }
    }
}
