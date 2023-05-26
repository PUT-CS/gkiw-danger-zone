use super::sound::SoundID;

pub enum AudioMessage {
    Play(SoundID, &'static str, bool),
    Stop(SoundID),
    Exit,
}

pub enum InternalMessage {
    Stop,
    Exit,
}
