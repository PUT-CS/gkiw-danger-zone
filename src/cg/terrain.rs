use super::{model::Model, shader::Shader};

pub enum TerrainType {
    Ocean,
    Desert,
}

pub struct Terrain {
    model: Model,
    type_: TerrainType
}

impl Terrain {
    pub fn new(path: &str, type_: TerrainType) -> Self {
        Terrain {
            model: Model::new(path),
            type_
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

