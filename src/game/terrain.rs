use cpu_time::ProcessTime;
use itertools::Itertools;
use lazy_static::lazy_static;
use log::{info, warn};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rayon::ThreadPool;
use std::collections::HashMap;
use std::ops::Range;
use std::thread;
use std::time::Instant;
use std::{mem::size_of_val, sync::mpsc};
use worldgen::{
    noise::perlin::PerlinNoise,
    noisemap::{
        NoiseMap, NoiseMapCombination, NoiseMapGenerator, NoiseMapGeneratorBase, ScaledNoiseMap,
        Seed, Size, Step,
    },
};

use crate::cg::{model::Model, shader::Shader};
const CHUNKS_X: i64 = 16;
const CHUNKS_Y: i64 = 16;
const CHUNK_X_RANGE: Range<i64> = -CHUNKS_X..CHUNKS_X + 1;
const CHUNK_Y_RANGE: Range<i64> = -CHUNKS_X..CHUNKS_X + 1;

use super::chunk::{self, Chunk};

pub const CHUNK_SIZE: i64 = 100;

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

pub type Position = (i64, i64);
pub type ChunkGenerator =
    NoiseMapCombination<NoiseMap<PerlinNoise>, ScaledNoiseMap<NoiseMap<PerlinNoise>>>;

pub struct Terrain {
    pub template_model: Box<Model>,
    generator: Box<ChunkGenerator>,
    chunk_map: HashMap<Position, Chunk>,
}

impl Default for Terrain {
    fn default() -> Self {
        Terrain::new(
            TERRAINS
                .get(&TerrainType::Desert)
                .expect("No path for that terrain"),
            TerrainType::Desert,
        )
    }
}

impl Terrain {
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

        let chunk_map = HashMap::new();

        Terrain {
            template_model: Box::new(Model::new(
                TERRAINS.get(&type_).expect("Path for terrain kind exists"),
            )),
            generator,
            chunk_map,
        }
    }

    pub unsafe fn draw(&self, shader: &Shader, request: &[Position]) {
        self.chunk_map
            .iter()
            .filter(|pair| request.contains(pair.0))
            .for_each(|pair| pair.1.draw(shader));
    }

    pub fn generate(&mut self) {
        let start = Instant::now();
        warn!("Generating terrain");
        let positions: Vec<Position> = CHUNK_X_RANGE.cartesian_product(CHUNK_Y_RANGE).collect();
        let chunks: Vec<(Position, Chunk)> = positions
            .par_iter()
            .map(|pos| (*pos, Chunk::new(pos, &self.generator, &self.template_model)))
            .collect();
        for mut pair in chunks {
            pair.1.model.reload_mesh();
            self.chunk_map.insert(pair.0, pair.1);
        }
        warn!(
            "Finished terrain generation. Took {}ms",
            start.elapsed().as_millis()
        );
    }
}
