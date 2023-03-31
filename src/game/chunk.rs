use crate::cg::model::Model;

use super::terrain::Position;

pub struct Chunk {
    model: Model
}

impl Chunk {
    pub fn new(model: Model) -> Self {
        Chunk { model }
    }
    pub fn apply_heights(&mut self, heights: Vec<Vec<f64>>) {
        assert_eq!(self.model.vertices.len(), heights.len() * heights[0].len());
        
    }
}
