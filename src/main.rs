use cg::shader::Shader;
use crate::game::flight::aircraft::AircraftKind::Mig21;
use game::player::Player;
use game::terrain::Terrain;
use glfw::Context;
use log::warn;
extern crate glfw;
use crate::game::game::Game;

const SCR_WIDTH: u32 = 1920;
const SCR_HEIGHT: u32 = 1080;

mod macros;
mod cg;
mod game;

fn main() {
    let mut first_mouse = true;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.;

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.;

    let mut game = Game::new();

    let shader = Shader::new(
        "src/shaders/model.vs",
        "src/shaders/fragment_transparent.fs",
    );

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
