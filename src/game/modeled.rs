use crate::cg::model::Model;

pub trait Modeled {
    fn model(&self) -> &Model;
    fn model_mut(&mut self) -> &mut Model;
}
