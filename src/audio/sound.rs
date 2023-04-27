use ambisonic::rodio::Decoder;
use std::{fs::File, io::BufReader};

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

pub type SoundID = u32;
