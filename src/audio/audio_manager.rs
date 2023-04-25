use ambisonic::{
    rodio::{Decoder, Sink, Source},
    Ambisonic, AmbisonicBuilder,
};
use cgmath::Point3;
use lazy_static::lazy_static;
use log::error;
use log::{info, warn};
use std::sync::atomic::Ordering;
use std::{
    collections::HashMap,
    path::Path,
    sync::{
        atomic::AtomicBool,
        mpsc::{self, Receiver, Sender},
        Arc,
    },
};
use std::{fs::File, io::BufReader};

pub enum AudioMessage {
    Play(SoundID, &'static str),
    Stop(SoundID),
    Resume(SoundID),
    Exit,
    MoveSoundTo(SoundID, Point3<f32>),
}

enum InternalMessage {
    Play(Sound),
    Stop,
    Resume,
    MoveSoundTo(Point3<f32>),
    Exit
}

pub struct Sound {
    pub source: Decoder<BufReader<File>>,
}

impl Sound {
    pub fn new(path: &str) -> Self {
        Self {
            source: Decoder::new(BufReader::new(File::open(path).unwrap())).unwrap(),
        }
    }
}

type SoundID = u32;

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
    receiver: Receiver<AudioMessage>,
    active_sounds: HashMap<SoundID, Sender<InternalMessage>>,
    scene: Ambisonic,
    /// End Of Work channel. A thread can signal that it ended playback
    /// and should be cleaned up from the AudioManager sound hashmap.
    eow: (Sender<SoundID>, Receiver<SoundID>),
}

impl AudioManager {
    pub fn run(receiver: Receiver<AudioMessage>) {
        // let file = File::open("resources/sounds/serce.mp3").unwrap();
        // let source = Decoder::new(BufReader::new(file)).unwrap();
        //scene.play_omni(source.convert_samples());

        let eow = mpsc::channel::<SoundID>();
        let scene = AmbisonicBuilder::default().build();
        let mut manager = Self {
            receiver,
            active_sounds: HashMap::new(),
            scene,
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
        info!("PT: Spawning an audio thread with id {id}");
        let s = Sink::new_idle();
        s.0.append(sound.source);
        s.0.play();
        loop {
            warn!("PT: WAITING FOR A MESSAGE");
            let message = match receiver.recv() {
                Ok(msg) => msg,
                Err(e) => {
                    error!("{e}");
                    panic!();
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
                    warn!("Killing player thread!");
                    break
                }
            }
            if s.0.empty() {
                eprintln!("Sink empty");
                eow_sender
                    .send(id)
                    .expect("Send message through EOW channel");
                break;
            }
        }
    }

    pub fn listen(&mut self) {
        loop {
            info!("AT: WAITING FOR A MESSAGE FROM MAIN THREAD");
            match self
                .receiver
                .recv()
                .expect("Receive message from the main thread")
            {
                AudioMessage::Play(id, path) => {
                    info!("AT: RECEIVED PLAY REQUEST");
                    // Channel for communicating with the new thread (pausing, resuming, moving sound position etc.)
                    let (sender, receiver) = mpsc::channel::<InternalMessage>();
                    // Sender that enables the thread to signal that its work has finished and it should get cleaned up
                    let eow_sender = self.eow.0.clone();
                    // Save the sound info to a hashmap, allowing later communication
                    self.active_sounds.insert(id, sender.clone());
                    info!("AT: SPAWNING PLAYER THREAD");
                    rayon::spawn(move || {
                        let sound = Sound::new(path);
                        AudioManager::player_thread(id, receiver, sound, eow_sender)
                    });
                    info!("AT: SPAWNED PLAYER THREAD");
                }
                AudioMessage::Resume(id) => todo!(),
                AudioMessage::Stop(id) => todo!(),
                AudioMessage::MoveSoundTo(id, position) => todo!(),
                AudioMessage::Exit => {
                    warn!("Audio cleanup");
                    self.active_sounds.values().for_each(|s| {s.send(InternalMessage::Exit).unwrap();});
                    break;
                }
            }
        }
    }

    pub fn play() {}
    pub fn stop() {}
    pub fn resume() {}
}
