use ambisonic::{
    rodio::{Decoder, Source},
    AmbisonicBuilder, Ambisonic,
};
use cgmath::Point3;
use gl::COMPRESSED_R11_EAC;
use log::{info, warn};
use std::{sync::{mpsc::{Receiver, Sender, self}, Arc, atomic::AtomicBool}, collections::HashMap, path::Path};
use std::{fs::File, io::BufReader};
use std::sync::atomic::Ordering;

pub enum AudioMessage {
    Play(SoundID, SoundID),
    Stop(SoundID),
    Resume(SoundID),
    Exit,
    MoveSoundTo(SoundID, Point3<f32>)
}

enum InternalMessage {
    Play(Sound),
    Stop,
    Resume,
    MoveSoundTo(Point3<f32>)
}

pub struct Sound {
    source: Decoder<BufReader<File>>
}
type SoundID = u32;

pub struct AudioManager {
    receiver: Receiver<AudioMessage>,
    active_sounds: HashMap<SoundID, Sender<InternalMessage>>,
    scene: Ambisonic,
    exit_flag: Arc<AtomicBool>,
    /// End Of Work channel. A thread can signal that it ended playback
    /// and should be cleaned up from the AudioManager sound hashmap.
    eow: (Sender<SoundID>, Receiver<SoundID>)
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
            exit_flag: Arc::new(AtomicBool::new(false)),
            eow
        };
        manager.listen();
    }

    // fn player_thread(id: SoundID, receiver: Receiver<InternalMessage>, sound: Sound, eow_sender: Sender<SoundID>) {
    //     //info!("Spawning an audio thread with sound: {:?}", sound.source);
    // }
    fn player_thread(id: SoundID, receiver: Receiver<InternalMessage>, sound: SoundID, eow_sender: Sender<SoundID>) {
        loop{}
        //info!("Spawning an audio thread with sound: {:?}", sound.source);
    }
    
    pub fn listen(&mut self) {
        loop {
            //dbg!(&self.active_sounds);
            //info!("Listening"); 
            match self.receiver.recv().expect("Receive message from the main thread") {
                AudioMessage::Play(id, sound) => {
                    // Channel for communicating with the new thread (pausing, resuming, moving sound position etc.)
                    let (sender, receiver) = mpsc::channel::<InternalMessage>();
                    // Sender that enables the thread to signal that its work has finished and it should get cleaned up
                    let eow_sender = self.eow.0.clone();
                    // Save the sound info to a hashmap, allowing later communication
                    self.active_sounds.insert(id, sender);
                    rayon::spawn(move || AudioManager::player_thread(id, receiver, sound, eow_sender));
                },
                AudioMessage::Resume(id) => todo!(),
                AudioMessage::Stop(id) => todo!(),
                AudioMessage::MoveSoundTo(id, position) => todo!(),
                AudioMessage::Exit => {
                    warn!("Audio cleanup");
                    self.exit_flag.store(true, Ordering::SeqCst);
                    break
                }
            }
        }
    }
    
    pub fn play() {}
    pub fn stop() {}
    pub fn resume() {}
}
