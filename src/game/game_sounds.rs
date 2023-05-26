use std::collections::HashMap;
use crate::audio::{sound::SoundID, audio_manager::SoundEffect};

pub struct SingleSounds {
    guns: SoundID,
    seeking: SoundID,
    locking: SoundID,
    locked: SoundID
}
