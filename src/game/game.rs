use crate::{SCR_HEIGHT, SCR_WIDTH};
use glfw::ffi::glfwSwapInterval;
use glfw::{Context, Glfw, Monitor, Window, WindowEvent};
use itertools::Itertools;
use log::info;
use std::sync::mpsc::Receiver;
extern crate glfw;
use self::glfw::{Action, Key};
use crate::cg::model::Model;
use crate::cg::shader::Shader;
use crate::cg::camera::Movement;
use cgmath::{perspective, vec3, Deg, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};
use std::ffi::CStr;

use super::terrain::Terrain;
use super::{enemy::Enemy, missile::Missile, player::Player};

pub struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    missiles: Vec<Missile>,
    terrain: Terrain,
    skybox: Model,
    pub glfw: Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
}

impl Game {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        info!("Initialized GLFW");
        log4rs::init_file("log_config.yaml", Default::default()).unwrap();
        info!("Initialized log4rs");

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(
                SCR_WIDTH,
                SCR_HEIGHT,
                "danger-zone",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        unsafe {
            glfwSwapInterval(0);
            gl::ClipControl(gl::LOWER_LEFT, gl::ZERO_TO_ONE);
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Enable(gl::DEPTH_TEST);
        }
        let mut terrain = Terrain::default();
        terrain.template_model.randomize_height();
        terrain.template_model.reload_mesh();
        Game {
            player: Player::default(),
            enemies: vec![],
            missiles: vec![],
            terrain,
            skybox: Model::new("resources/objects/skybox/skybox.obj"),
            glfw,
            window,
            events,
        }
    }

    /// Compute new positions of all game objects based on input and state of the game
    pub fn update(&mut self, delta_time: f32) {
        self.player.apply_controls(delta_time * 200.);
        self.player.aircraft_mut().apply_decay();
    }

    /// Check if the player
    pub fn targeted_enemies(&self) -> Option<Vec<Player>> {
        let player_front = self.player.camera().front;
        let player_position = self.player.camera().position;

        // temporary. self.enemies here later
        let enemies = vec![self.player.clone()];

        let targeted: Vec<Player> = enemies
            .iter()
            .filter(|enemy| {
                let pos = enemy.aircraft().model().position;
                let direction = (pos - player_position).normalize();
                let deg = direction.angle(player_front).0.to_degrees();

                deg < 5.
            })
            .map(|p| p.to_owned())
            .collect();

        (!targeted.is_empty()).then(|| targeted)
    }

    pub unsafe fn draw(&mut self, shader: &Shader) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        shader.use_program();

        shader.set_mat4(
            c_str!("projection"),
            &perspective(Deg(45.), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 30000.0),
        );
        shader.set_mat4(c_str!("view"), &self.player.camera().view_matrix());

        let mut model_matrix =
            self.player.aircraft().model().model_matrix() * Matrix4::from_scale(10.0);
        //model_matrix = model_matrix * Matrix4::from_axis_angle(Vector3::unit_y(), Deg(0.));
        //model_matrix = model_matrix * Matrix4::from_translation(vec3(0.0, 0., 4.0));
        //model_matrix = model_matrix * Matrix4::from_translation(vec3(0.0, 0., 0.0));
        model_matrix = model_matrix * Matrix4::from_angle_y(Deg(180.));
        shader.set_mat4(c_str!("model"), &model_matrix);
        //self.player.draw(&shader);

        let mut model_matrix = Matrix4::<f32>::from_value(1.0);
        model_matrix = model_matrix * Matrix4::from_scale(15.0);
        shader.set_mat4(c_str!("model"), &model_matrix);
        self.terrain.draw(&shader);
	
        let mut model_matrix = Matrix4::<f32>::from_value(1.0);
        model_matrix = model_matrix * Matrix4::from_scale(10000.0);
        shader.set_mat4(c_str!("model"), &model_matrix);
        self.skybox.draw(&shader);

        let mut model_matrix = self.player.cockpit.model_matrix()
            * Matrix4::from_translation(vec3(0.0, -0.3, 0.0))
            * Matrix4::from_scale(0.5)
            * Matrix4::from_angle_y(Deg(-90.));
        let time = self.glfw.get_time() as f32 * 2.0;
        model_matrix = model_matrix
            * Matrix4::from_translation(vec3(
                time.sin() * 0.01,
                time.cos().sin() * 0.01,
                time.cos() * 0.01,
            ));
        shader.set_mat4(c_str!("model"), &model_matrix);
        shader.set_mat4(c_str!("view"), &Matrix4::from_value(1.0));
        //self.player.cockpit.draw(&shader);
    }

    pub fn process_events(
        &mut self,
        first_mouse: &mut bool,
        last_x: &mut f32,
        last_y: &mut f32,
        //camera: &mut Camera,
    ) {
        for (_, event) in glfw::flush_messages(&self.events) {
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

    /// First level of controls. Captures pressed keys and calls appropriate functions.
    /// Additionaly, set all decays on the aircraft as true.
    pub fn process_key(&mut self) {
        self.player_mut()
            .aircraft_mut()
            .controls_mut()
            .set_all_decays(true);
        let delta_time = 1.;
        key_pressed!(self.window, Key::Escape, self.window.set_should_close(true));
        key_pressed!(
            self.window,
            Key::W,
            self.player.process_key(Movement::PitchDown, delta_time)
        );
        key_pressed!(
            self.window,
            Key::S,
            self.player.process_key(Movement::PitchUp, delta_time)
        );
        key_pressed!(
            self.window,
            Key::A,
            self.player.process_key(Movement::RollLeft, delta_time)
        );
        key_pressed!(
            self.window,
            Key::D,
            self.player.process_key(Movement::RollRight, delta_time)
        );
        key_pressed!(
            self.window,
            Key::E,
            self.player.process_key(Movement::YawRight, delta_time)
        );
        key_pressed!(
            self.window,
            Key::Q,
            self.player.process_key(Movement::YawLeft, delta_time)
        );
        key_pressed!(
            self.window,
            Key::LeftShift,
            self.player.process_key(Movement::ThrottleUp, delta_time)
        );
        key_pressed!(
            self.window,
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
}
