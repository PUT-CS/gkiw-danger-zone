use crate::cg::{model::Model, shader::Shader};

pub struct Chunk {
    model: Model,
}

impl Chunk {
    pub fn new(model: Model) -> Self {
        Chunk { model }
    }
    pub fn apply_heights(&mut self, heights: Vec<Vec<f64>>) {
        assert!(!heights.is_empty());
        assert_eq!(self.model.vertices.len(), heights.len() * heights[0].len());
    }
}

impl Chunk {
    pub fn draw(&self, shader: &Shader) {
        unsafe {
            self.model.draw(shader);
        }
    }
}
