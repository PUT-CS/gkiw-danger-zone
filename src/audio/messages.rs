use cgmath::Point3;

use super::sound::{Sound, SoundID};

pub enum AudioMessage {
    Play(SoundID, &'static str, bool),
    Pause(SoundID),
    Stop(SoundID),
    Resume(SoundID),
    Exit,
    MoveSoundTo(SoundID, Point3<f32>),
}

pub enum InternalMessage {
    Play(Sound, bool),
    Stop,
    Resume,
    MoveSoundTo(Point3<f32>),
    Exit,
}
