use crate::cg::{model::Model, shader::Shader};
use lazy_static::lazy_static;
use log::{info, warn};
use std::collections::HashMap;
use super::drawable::Drawable;

lazy_static! {
    static ref TERRAINS: HashMap<TerrainType, &'static str> = HashMap::from([(
        TerrainType::Desert,
        "resources/objects/terrain/terrain_new.obj"
    )]);
}

#[derive(Hash, PartialEq, Eq)]
pub enum TerrainType {
    Ocean,
    Desert,
}

pub struct Terrain {
    pub model: Model,
}

impl Terrain {
    pub fn new(path: &str, type_: TerrainType) -> Self {
        info!("Creating new Terrain with template: {path}",);
        Terrain {
            model: Model::new(TERRAINS.get(&type_).expect("Path for terrain kind exists")),
        }
    }
}

impl Default for Terrain {
    fn default() -> Self {
        let path = TERRAINS
            .get(&TerrainType::Desert)
            .expect("No path for that terrain");
        warn!("{}", path);
        Terrain::new(&path, TerrainType::Desert)
    }
}

impl Drawable for Terrain {
    unsafe fn draw(&self, shader: &Shader) {
        self.model.draw(shader);
    }
}
