use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use worldgen::{
    noise::perlin::PerlinNoise,
    noisemap::{NoiseMap, NoiseMapCombination, NoiseMapGenerator, ScaledNoiseMap, Seed, Step, NoiseMapGeneratorBase, Size},
};

use crate::cg::{model::Model, shader::Shader};

use super::chunk::Chunk;

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

pub struct Terrain {
    pub template_model: Model,
    generator:
        Box<NoiseMapCombination<NoiseMap<PerlinNoise>, ScaledNoiseMap<NoiseMap<PerlinNoise>>>>,
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

        Terrain {
            template_model: Model::new(TERRAINS.get(&type_).expect("Path for terrain kind exists")),
            generator,
            chunk_map: HashMap::new(),
        }
    }
    pub unsafe fn draw(&self, shader: &Shader) {
        self.template_model.draw(shader);
    }
    pub fn generate_chunk(&mut self, pos: Position) {
        let chunk_heights = self.generator.generate_sized_chunk(Size::of(100, 100), pos.0, pos.1);
        let mut chunk = Chunk::new(self.template_model.clone());
        chunk.apply_heights(chunk_heights);
    }
}
