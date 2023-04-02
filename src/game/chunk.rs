use std::{time::Duration, fmt::Debug};

use log::info;
use worldgen::noisemap::{NoiseMapGeneratorBase, Size};

use crate::cg::{model::Model, shader::Shader};

use super::terrain::{ChunkGenerator, Position, CHUNK_SIZE};

pub struct Chunk {
    pub model: Model,
}

impl Chunk {
    pub fn new(pos: &Position, generator: &ChunkGenerator, model: &Model) -> Self {
        //info!("Generating chunk at {pos:?}");
        let heights =
            generator.generate_sized_chunk(Size::of(CHUNK_SIZE, CHUNK_SIZE), pos.0, pos.1);
        assert!(!heights.is_empty());
        let mut model = model.clone();
        let mut idx = 0;
        heights.iter().for_each(|row| row.iter().for_each(|&number| {
            model.vertices[idx].position.y += number as f32 / 10.;
            idx += 1;
        }));
        //info!("Finished chunk at {pos:?}");
        Chunk { model }
    }
}

impl Chunk {
    pub fn draw(&self, shader: &Shader) {
        unsafe {
            self.model.draw(shader);
        }
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Chunk with model at pos {:?}", self.model.position);
        Ok(())
    }
    
}
