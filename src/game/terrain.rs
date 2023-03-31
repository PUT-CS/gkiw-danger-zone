use itertools::Itertools;
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use std::thread;
use std::{mem::size_of_val, sync::mpsc};
use worldgen::{
    noise::perlin::PerlinNoise,
    noisemap::{
        NoiseMap, NoiseMapCombination, NoiseMapGenerator, NoiseMapGeneratorBase, ScaledNoiseMap,
        Seed, Size, Step,
    },
};

use crate::cg::{model::Model, shader::Shader};

use super::chunk::{Chunk, self};

const CHUNK_SIZE: i64 = 100;

lazy_static! {
    static ref TERRAINS: HashMap<TerrainType, &'static str> =
    HashMap::from([(TerrainType::Desert, "resources/objects/terrain/terrain2.obj")]);
    //HashMap::from([(TerrainType::Desert, "resources/objects/skybox/skybox.obj")]);
}

#[derive(Hash, PartialEq, Eq)]
pub enum TerrainType {
    Ocean,
    Desert,
}

pub enum ChunkResult<'a> {
    Ok(&'a Chunk),
    None(Position),
}

impl<'a> ChunkResult<'a> {
    pub fn is_ok(&self) -> bool {
        matches!(*self, ChunkResult::Ok(_))
    }
    pub fn is_none(&self) -> bool {
        matches!(*self, ChunkResult::None(_))
    }
}

pub type Position = (i64, i64);

pub struct Terrain<'a> {
    pub template_model: Box<Model>,
    generator:
        Box<NoiseMapCombination<NoiseMap<PerlinNoise>, ScaledNoiseMap<NoiseMap<PerlinNoise>>>>,
    chunk_map: HashMap<Position, &'a Chunk>,
}

impl<'a> Default for Terrain<'a> {
    fn default() -> Self {
        Terrain::new(
            TERRAINS
                .get(&TerrainType::Desert)
                .expect("No path for that terrain"),
            TerrainType::Desert,
        )
    }
}

impl<'a> Terrain<'a> {
    pub fn new(path: &str, type_: TerrainType) -> Self {
        info!("Creating new Terrain with template: {path}",);
        let noise = PerlinNoise::new();
        let nm1 = NoiseMap::new(noise)
            .set_seed(Seed::of("DangerZone"))
            .set_step(Step::of(0.005, 0.005));
        let nm2 = NoiseMap::new(noise)
            .set_seed(Seed::of("TerrainGen"))
            .set_step(Step::of(0.05, 0.05));
        let generator = Box::new(nm1 + nm2 * 4);
        generator.set_size(Size {
            w: CHUNK_SIZE,
            h: CHUNK_SIZE,
        });

        Terrain {
            template_model: Box::new(Model::new(
                TERRAINS.get(&type_).expect("Path for terrain kind exists"),
            )),
            generator,
            chunk_map: HashMap::new(),
        }
    }

    pub unsafe fn draw(&self, shader: &Shader) {
        self.template_model.draw(shader);
    }

    pub fn draw2(&self, shader: &Shader, positions: Vec<Position>) {

        // Check which chunks were successfuly obtained from the chunk map
        let chunk_results = positions.iter().map(|&pos| match self.chunk_map.get(&pos) {
            Some(chunk) => ChunkResult::Ok(chunk),
            None => ChunkResult::None(pos),
        });

        // Split the results into two vectors.
        // 
        
        //Successful 
        let mut chunks: Vec<&Chunk> = chunk_results
            .clone()
            .filter(|res| res.is_ok())
            .map(|res| match res {
                ChunkResult::Ok(chunk) => chunk,
                _ => unreachable!(),
            })
            .collect();
        
        // Unsuccessful
        let requests: Vec<Position> = chunk_results
            .filter(|res| res.is_none())
            .map(|res| match res {
                ChunkResult::None(pos) => pos,
                _ => unreachable!(),
            })
            .collect();

        let mut children = vec![];
        let length = requests.len();

        // Create the sender and receiver.
        // Each thread will handle a different request and send the created Chunk to the receiver.
        let (tx, rx) = mpsc::channel::<Chunk>();
        
        for position in requests {
            // Each thread will be given its own copy of the sender object
            let thread_tx = tx.clone();
            // As well as the generator
            let generator = self.generator.clone();
            // And the template model (this copies, so it's subject to improvement) TODO
            let template = self.template_model.clone();
            // Spawn the worker thread
            let child = thread::spawn(move || {
                let mut chunk = Chunk::new(*template);
                let heights = generator.generate_sized_chunk(
                    Size {
                        w: CHUNK_SIZE,
                        h: CHUNK_SIZE,
                    },
                    position.0,
                    position.1,
                );
                chunk.apply_heights(heights);
                // Send the chunk down the channel, exit
                thread_tx.send(chunk).expect("Send chunk");
                info!("Generated chunk at {position:?}");
            });
            // Add the thread handle to a vec so it can be joined later. (avoids exiting before finishing work)
            children.push(child);
        }
        // We will temporarily put received chunks in Boxes (may be improved) TODO
        let mut tmp_boxes = vec![];

        // Receive all chunks. rx.recv() is blocking, so we can be sure that it receives the required amoun of chunks,
        // no additional logic required
        for _ in 0..length {
            let received = rx.recv().expect("Receive chunk");
            tmp_boxes.push(Box::new(received));
        }

        // Join all the spawned threads
        for child in children {
            child.join().expect("Join thread");
        }

        // Convert Boxes to references and join both vectors
        let mut new_chunks: Vec<&Chunk> = tmp_boxes.iter().map(|b| b.as_ref()).collect();
        chunks.append(&mut new_chunks);

        // Draw all the requested chunks
        for chunk in chunks {
            chunk.draw(shader);
        }
    }
}
