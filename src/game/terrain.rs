use super::{drawable::Drawable, modeled::Modeled};
use crate::cg::{model::Model, shader::Shader};
use cgmath::Point2;
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use std::ops::Range;

lazy_static! {
    static ref TERRAINS: HashMap<TerrainType, &'static str> =
        //HashMap::from([(TerrainType::Desert, "resources/objects/terrain/teren2.obj")]);
        HashMap::from([(TerrainType::Desert, "resources/objects/terrain/terrain.obj")]);
}

#[derive(Hash, PartialEq, Eq)]
pub enum TerrainType {
    Desert,
}

pub struct Bounds {
    pub x: Range<i32>,
    pub z: Range<i32>,
}

pub struct Terrain {
    pub model: Model,
    pub heights: HashMap<Point2<i32>, f32>,
    pub bounds: Bounds,
}

impl Terrain {
    pub fn new(path: &str, type_: TerrainType) -> Self {
        info!("Creating new Terrain with template: {path}",);
        let model = Model::new(TERRAINS.get(&type_).expect("Path for terrain kind exists"));
        let heights = Terrain::heights_of(&model);
        let bounds = model.bounds();
        Terrain {
            model,
            heights,
            bounds,
        }
    }
    fn heights_of(terrain_model: &Model) -> HashMap<Point2<i32>, f32> {
        let positions = terrain_model.vertices.iter().map(|v| v.position);
        let defined_points: HashMap<Point2<i32>, f32> = HashMap::from_iter(
            positions
                .clone()
                .map(|v| ((v.x as i32, v.z as i32).into(), v.y)),
        );
        defined_points
    }
    pub fn height_at(&self, pos: &Point2<i32>) -> f32 {
        *self.heights.get(pos).unwrap_or(&0.) + self.model.transformation.translation.y + 0.8
    }
}

impl Default for Terrain {
    fn default() -> Self {
        let path = TERRAINS
            .get(&TerrainType::Desert)
            .expect("No path for that terrain");
        Terrain::new(path, TerrainType::Desert)
    }
}

impl Drawable for Terrain {
    unsafe fn draw(&self, shader: &Shader) {
        self.model().draw(shader);
    }
}

impl Modeled for Terrain {
    fn model(&self) -> &Model {
        &self.model
    }
    fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }
}
