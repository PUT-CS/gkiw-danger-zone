use cg::shader::Shader;
use glfw::Context;
extern crate glfw;
use crate::game::game::Game;

const SCR_WIDTH: u32 = 1000;
const SCR_HEIGHT: u32 = 1000;

mod macros;
mod cg {
    pub mod camera;
    pub mod model;
    pub mod shader;
    pub mod terrain;
    pub mod transformation;
}
mod game {
    pub mod enemy;
    pub mod game;
    pub mod missile;
    pub mod flight {
        pub mod aircraft;
        pub mod control_surfaces;
        pub mod spec;
        pub mod steerable;
    }
    pub mod player;
}

fn main() {
    let mut first_mouse = true;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.;

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.;

    let mut game = Game::new();

    let shader = Shader::new("src/shaders/model.vs", "src/shaders/model.fs");

    while !game.window.should_close() {
        let current_frame = game.glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;
        game.process_events(&mut first_mouse, &mut last_x, &mut last_y);
        game.process_key();

        unsafe {
            game.update(delta_time);
            game.draw(&shader);
        }

        game.window.swap_buffers();
        game.glfw.poll_events();
    }
}
