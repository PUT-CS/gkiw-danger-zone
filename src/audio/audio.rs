use std::sync::mpsc::Sender;

use super::audio_manager::{SoundEffect, SOUNDS};
use super::messages::AudioMessage;
use super::sound::SoundID;
use crate::game::game::ID_GENERATOR;
use crate::game::id_gen::IDKind;

pub struct Audio {
    sender: Sender<AudioMessage>,
}

impl Audio {
    pub fn new(sender: Sender<AudioMessage>) -> Self {
        Self { sender }
    }
    pub fn play(&self, effect: SoundEffect, repeat: bool) -> SoundID {
        let id = ID_GENERATOR.lock().unwrap().get_new_id_of(IDKind::Sound);
        let path = SOUNDS.get(&effect).unwrap();
        self.sender
            .send(AudioMessage::Play(id, path, repeat))
            .expect("Send message to audio thread");
        id
    }
    pub fn pause(&self, id: SoundID) {
        self.sender.send(AudioMessage::Pause(id)).unwrap()
    }
    pub fn stop(&self, id: SoundID) {
        self.sender.send(AudioMessage::Stop(id)).unwrap()
    }
    pub fn exit_hook(&self) {
        self.sender
            .send(AudioMessage::Exit)
            .expect("Send Exit message to audio thread");
    }
}
