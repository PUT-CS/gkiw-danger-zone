use std::collections::HashMap;

use super::{model::Model, shader::Shader};
use lazy_static::lazy_static;
use log::info;

lazy_static! {
    static ref TERRAINS: HashMap<TerrainType, &'static str> =
        HashMap::from([(TerrainType::Desert, "resources/objects/terrain/terrain.obj")]);
}

#[derive(Hash, PartialEq, Eq)]
pub enum TerrainType {
    Ocean,
    Desert,
}

pub struct Terrain {
    model: Model,
    type_: TerrainType,
}

impl Default for Terrain {
    fn default() -> Self {
        Terrain::new(TERRAINS.get(&TerrainType::Desert).expect("No path for that terrain"), TerrainType::Desert)
    }
}

impl Terrain {
    pub fn new(path: &str, type_: TerrainType) -> Self {
        info!("Creating new Terrain: {path}",);
        Terrain {
            model: Model::new(path),
            type_,
        }
    }
    pub fn generate(&mut self) {
        // apply a height map / random noise map to generate a terrain
        // the model attribute has a vertices attribute.
        // each Vertex there has a position vector.
        // Modify the appropriate element of that position
        // vector to alter vertex height
    }
    pub unsafe fn draw(&self, shader: &Shader) {
        self.model.draw(shader);
    }
}
