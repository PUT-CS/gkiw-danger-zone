use glfw::ffi::glfwSwapInterval;
use glfw::{Context, Glfw, Window, WindowEvent};
use log::info;
use std::sync::mpsc::Receiver;
extern crate glfw;
use self::glfw::{Action, Key};
use crate::cg::model::Model;
use crate::cg::shader::Shader;
use crate::cg::{camera::Movement, terrain::Terrain};
use cgmath::{vec3, Deg, Matrix4, Vector3};
use std::ffi::CStr;

use super::{enemy::Enemy, missile::Missile, player::Player};

const SCR_WIDTH: u32 = 1000;
const SCR_HEIGHT: u32 = 1000;

pub struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    missiles: Vec<Missile>,
    terrain: Terrain,
    skybox: Model,
    enemy_id: u32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: Player::default(),
            enemies: vec![],
            missiles: vec![],
            terrain: Terrain::default(),
            skybox: Model::new("resources/objects/skybox/skybox.obj"),
            enemy_id: 0,
        }
    }

    pub fn update(&mut self) {
        dbg!(&self.player.aircraft().controls());
        self.player.apply_controls();
        self.player.aircraft_mut().apply_decay();
    }

    pub unsafe fn draw(&mut self, shader: &Shader) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        shader.use_program();

        shader.set_mat4(
            c_str!("projection"),
            self.player.camera().projection_matrix(),
        );
        shader.set_mat4(c_str!("view"), &self.player.camera().view_matrix());

        let mut model_matrix = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, -1.0));
        model_matrix = model_matrix * Matrix4::from_axis_angle(Vector3::unit_x(), Deg(0.));
        shader.set_mat4(c_str!("model"), &model_matrix);
        self.player.draw(&shader);

        let mut model_matrix = Matrix4::<f32>::from_translation(vec3(0.0, -10.0, 0.0));
        model_matrix = model_matrix * Matrix4::from_scale(10000.0);
        shader.set_mat4(c_str!("model"), &model_matrix);
        self.terrain.draw(&shader);

        let mut model_matrix = Matrix4::<f32>::from_translation(vec3(0.0, -10.0, 0.0));
        model_matrix = model_matrix * Matrix4::from_scale(10000.0);
        shader.set_mat4(c_str!("model"), &model_matrix);
        self.skybox.draw(&shader);
    }

    pub fn process_events(
        &mut self,
        events: &Receiver<(f64, glfw::WindowEvent)>,
        first_mouse: &mut bool,
        last_x: &mut f32,
        last_y: &mut f32,
        //camera: &mut Camera,
    ) {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let (xpos, ypos) = (xpos as f32, ypos as f32);
                    if *first_mouse {
                        *last_x = xpos;
                        *last_y = ypos;
                        *first_mouse = false;
                    }

                    let xoffset = xpos - *last_x;
                    // reversed since y-coordinates go from bottom to top
                    let yoffset = *last_y - ypos;

                    *last_x = xpos;
                    *last_y = ypos;

                    self.player
                        .camera_mut()
                        .process_mouse_movement(xoffset, yoffset);
                }
                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    self.player
                        .camera_mut()
                        .process_mouse_scroll(yoffset as f32);
                }
                _ => {}
            }
        }
    }

    pub fn process_key(&mut self, window: &mut glfw::Window, delta_time: f32) {
        self.player_mut()
            .aircraft_mut()
            .controls_mut()
            .set_all_decays(true);
        key_pressed!(window, Key::Escape, window.set_should_close(true));
        key_pressed!(
            window,
            Key::W,
            self.player.process_key(Movement::PitchDown, delta_time)
        );
        key_pressed!(
            window,
            Key::S,
            self.player.process_key(Movement::PitchUp, delta_time)
        );
        key_pressed!(
            window,
            Key::A,
            self.player.process_key(Movement::RollLeft, delta_time)
        );
        key_pressed!(
            window,
            Key::D,
            self.player.process_key(Movement::RollRight, delta_time)
        );
        key_pressed!(
            window,
            Key::E,
            self.player.process_key(Movement::YawRight, delta_time)
        );
        key_pressed!(
            window,
            Key::Q,
            self.player.process_key(Movement::YawLeft, delta_time)
        );
        key_pressed!(
            window,
            Key::LeftShift,
            self.player.process_key(Movement::ThrottleUp, delta_time)
        );
        key_pressed!(
            window,
            Key::LeftControl,
            self.player.process_key(Movement::ThrottleDown, delta_time)
        );
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn set_player(&mut self, p: Player) {
        self.player = p;
    }

    pub fn init() -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
        log4rs::init_file("log_config.yaml", Default::default()).unwrap();
        info!("Initialized log4rs");

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        info!("Initialized GLFW");

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(
                SCR_WIDTH,
                SCR_HEIGHT,
                "LearnOpenGL",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        unsafe {
            glfwSwapInterval(0);
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Enable(gl::DEPTH_TEST);
        }

        (glfw, window, events)
    }
}
