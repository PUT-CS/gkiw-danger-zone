/// Trait for everything that can be mutated in the sense of Eulerian angles
pub trait Steerable {
    fn pitch(&mut self, amount: f32);
    fn yaw(&mut self, amount: f32);
    fn roll(&mut self, amount: f32);
    fn forward(&mut self, amount: f32);
}
