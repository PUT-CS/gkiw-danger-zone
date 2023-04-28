use super::{
    messages::{InternalMessage, AudioMessage},
    sound::{Sound, SoundID},
};
use ambisonic::rodio::{OutputStream, Sink};
use lazy_static::lazy_static;
use log::{error, info, warn};
use std::{
    collections::HashMap,
    sync::mpsc::TryRecvError,
    sync::mpsc::{self, Receiver, Sender},
};

#[derive(Hash, PartialEq, Eq)]
pub enum SoundEffect {
    JetEngine,
    Beep,
}

lazy_static! {
    pub static ref SOUNDS: HashMap<SoundEffect, &'static str> =
        HashMap::from([(SoundEffect::Beep, "resources/sounds/beep.mp3")]);
}

pub struct AudioManager {
    /// Receiver for reading requests coming from the main game thread
    receiver: Receiver<AudioMessage>,
    /// HashMap containing all currently played sounds with a sender
    /// allowing for communication with the worker thread
    active_sounds: HashMap<SoundID, Sender<InternalMessage>>,
    /// End Of Work channel. A thread can signal that it ended playback
    /// and should be cleaned up from the AudioManager sound hashmap.
    eow: (Sender<SoundID>, Receiver<SoundID>),
}

impl AudioManager {
    /// Create a new AudioManager and start listening for messages
    pub fn run(receiver: Receiver<AudioMessage>) {
        let eow = mpsc::channel::<SoundID>();
        let mut manager = Self {
            receiver,
            active_sounds: HashMap::new(),
            eow,
        };
        manager.listen();
    }

    fn player_thread(
        id: SoundID,
        receiver: Receiver<InternalMessage>,
        sound: Sound,
        eow_sender: Sender<SoundID>,
    ) {
        info!("Spawning audio thread with id: {id}");
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(sound.source);
        sink.play();
        loop {
            let message = match receiver.try_recv() {
                Ok(m) => m,
                Err(TryRecvError::Disconnected) => {
                    error!("Receiver disconnected");
                    panic!();
                }
                Err(TryRecvError::Empty) => {
                    if sink.empty() {
                        info!("Sound finished");
                        eow_sender
                            .send(id)
                            .expect("Send message through EOW channel");
                        break;
                    }
                    continue;
                }
            };
            match message {
                InternalMessage::Play(_) => {
                    error!("Sent a play request to an already playing thread")
                }
                InternalMessage::MoveSoundTo(_pos) => todo!("Move sound"),
                InternalMessage::Resume => todo!("Resume"),
                InternalMessage::Stop => todo!("Stop sound"),
                InternalMessage::Exit => {
                    info!("Killing player thread!");
                    break;
                }
            }
        }
    }

    /// Listen to messages on two receivers.
    /// 1. Messages from the main thread - play, resume, stop requests
    /// 2. Messages from worker threads that finished playback
    pub fn listen(&mut self) {
        loop {
            match self.receiver.try_recv() {
                Ok(msg) => {
                    if self.handle_audio_message_or_break(msg) {
                        break;
                    }
                }
                Err(TryRecvError::Disconnected) => {
                    error!("Sender Disconnected");
                    panic!();
                }
                Err(TryRecvError::Empty) => {}
            };
            match self.eow.1.try_recv() {
                Ok(id) => {
                    info!("Removing sound with id: {id}");
                    self.active_sounds.remove(&id);
                }
                Err(TryRecvError::Disconnected) => {
                    error!("Sender disconnected");
                    panic!()
                }
                Err(TryRecvError::Empty) => {}
            }
        }
    }

    fn handle_audio_message_or_break(&mut self, msg: AudioMessage) -> bool {
        match msg {
            AudioMessage::Play(id, path) => {
                // Channel for communicating with the new thread (pausing, resuming, moving sound position etc.)
                let (sender, receiver) = mpsc::channel::<InternalMessage>();
                // Sender that enables the thread to signal that its work has finished and it should get cleaned up
                let eow_sender = self.eow.0.clone();
                // Save the sound info to a hashmap, allowing later communication
                self.active_sounds.insert(id, sender.clone());
                rayon::spawn(move || {
                    let sound = Sound::new(path);
                    AudioManager::player_thread(id, receiver, sound, eow_sender)
                });
                false
            }
            AudioMessage::Resume(id) => todo!(),
            AudioMessage::Stop(id) => todo!(),
            AudioMessage::MoveSoundTo(id, position) => todo!(),
            AudioMessage::Exit => {
                info!("Starting audio cleanup");
                self.active_sounds.values().for_each(|s| {
                    s.send(InternalMessage::Exit).unwrap();
                });
                true
            }
        }
    }

    pub fn play() {
        todo!()
    }
    pub fn stop() {
        todo!()
    }
    pub fn resume() {
        todo!()
    }
}
