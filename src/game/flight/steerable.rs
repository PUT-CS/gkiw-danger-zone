pub trait Steerable {
    fn pitch(&mut self, amount: f32);
    fn yaw(&mut self, amount: f32);
    fn roll(&mut self, amount: f32);
    fn forward(&mut self, amount: f32);
    //fn throttle_down(&mut self, amount: f32);
}
