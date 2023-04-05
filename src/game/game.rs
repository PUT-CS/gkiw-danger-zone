use crate::{SCR_HEIGHT, SCR_WIDTH};
use glfw::ffi::glfwSwapInterval;
use glfw::{Context, Glfw, Window, WindowEvent};
use itertools::Itertools;
use log::info;
use log::warn;
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
extern crate glfw;
use self::glfw::{Action, Key};
use super::missile::EnemyID;
use super::terrain::Terrain;
use super::{enemy::Enemy, missile::Missile, player::Player};
use crate::cg::camera::Movement;
use crate::cg::model::Model;
use crate::cg::shader::Shader;
use crate::game::drawable::Drawable;
use crate::game::flight::aircraft::AircraftKind::Mig21;
use crate::game::id_gen::{IDGenerator, IDKind};
use cgmath::Vector3;
use cgmath::{perspective, vec3, Deg, InnerSpace, Matrix4, SquareMatrix};
use std::ffi::CStr;

const TARGET_ENEMIES: usize = 4;

pub struct Game {
    player: Player,
    enemies: HashMap<EnemyID, Enemy>,
    missiles: Vec<Missile>,
    terrain: Terrain,
    skybox: Model,
    id_generator: IDGenerator,
    pub glfw: Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
}

impl Game {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        log4rs::init_file("log_config.yaml", Default::default()).unwrap();
        info!("Initialized log4rs");
        info!("Initialized GLFW");

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
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        ThreadPoolBuilder::new()
            .num_threads(256)
            .build_global()
            .expect("Configure global rayon threadpool");

        let mut terrain = Terrain::default();
        terrain.model.scale(0.005);

        let mut player = Player::default();
        player.aircraft_mut().model_mut().scale(15.);

        player
            .cockpit_mut()
            .translate(vec3(0.0, -0.3, 0.0))
            .scale(0.5)
            .rotate(Vector3::unit_y(), Deg(-90.));

        let mut skybox = Model::new("resources/objects/skybox/skybox.obj");
        skybox.scale(1000.);

        Game {
            player,
            enemies: HashMap::with_capacity(TARGET_ENEMIES),
            missiles: vec![],
            terrain,
            skybox,
            id_generator: IDGenerator::default(),
            glfw,
            window,
            events,
        }
    }

    /// Compute new positions of all game objects based on input and state of the game
    pub fn update(&mut self, delta_time: f32) {
        self.player.apply_controls(delta_time * 200.);
        self.player.aircraft_mut().apply_decay();
        // currently there will be 4 enemies stacked in one spot
        self.respawn_enemies();
    }

    pub fn respawn_enemies(&mut self) {
        let diff = TARGET_ENEMIES - self.enemies.len();
        if diff > 0 {
            warn!("Respawning {diff} enemies");
        }
        (0..diff)
            .map(|_| {
                (
                    self.id_generator.get_new_id_of(IDKind::Enemy),
                    Enemy::new(Mig21),
                )
            })
            .for_each(|t| {
                self.enemies.insert(t.0, t.1);
            });
    }

    /// Check if the player aims their nose at an enemy, triggering a missile lock
    /// countdown on one of them (lock not implemented yet)
    pub fn targeted_enemy_id(&self) -> Option<EnemyID> {
        let player_front = self.player.camera().front;
        let player_position = self.player.camera().position;

        let mut targeted = self
            .enemies
            .iter()
            .map(|tuple| {
                let enemy = tuple.1;
                let pos = enemy.aircraft().model().position;
                let direction = (pos - player_position).normalize();
                let deg = direction.angle(player_front).0.to_degrees();
                (tuple.0, (deg, enemy))
            })
            .collect_vec();
        if targeted.is_empty() {
            return None;
        }
        targeted.sort_by(|t1, t2| t2.1 .0.partial_cmp(&t2.1 .0).unwrap());
        Some(*targeted[0].0)
    }

    pub unsafe fn draw(&mut self, shader: &Shader) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        shader.use_program();

        shader.set_mat4(
            c_str!("projection"),
            &perspective(Deg(45.), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 30000.0),
        );
        shader.set_mat4(c_str!("view"), &self.player.camera().view_matrix());

        // Drawing game objects starts here
        let mut model_matrix = self.terrain.model.model_matrix();
        shader.set_mat4(c_str!("model"), &model_matrix);
        self.terrain.draw(&shader);

        model_matrix = self.skybox.model_matrix();
        shader.set_mat4(c_str!("model"), &model_matrix);
        self.skybox.draw(&shader);

        model_matrix = self
            .enemies
            .get_mut(&0)
            .unwrap()
            .aircraft_mut()
            .model()
            .model_matrix();
        shader.set_mat4(c_str!("model"), &model_matrix);
        self.enemies.get(&0).unwrap().draw(&shader);

        model_matrix = self.player.cockpit.model_matrix();
        let time = self.glfw.get_time() as f32 * 2.0;
        model_matrix = model_matrix
            * Matrix4::from_translation(vec3(
                time.sin() * 0.01,
                time.cos().sin() * 0.01,
                time.cos() * 0.01,
            ));
        shader.set_mat4(c_str!("model"), &model_matrix);
        shader.set_mat4(c_str!("view"), &Matrix4::identity());
        self.player.cockpit.draw(&shader);
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
        key_pressed!(self.window, Key::Space, self.launch_missile())
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    /// Perform all actions necessary to launch a missile
    pub fn launch_missile(&mut self) {
        let target: Option<EnemyID> = self.targeted_enemy_id();
        self.spawn_missile(target);
    }

    /// Give the missiles a reference to the Enemy they are currently
    /// targeting so they can mutate their state accordingly
    pub fn update_missile(&mut self) {}

    /// Create a new missile and add it to the self.missiles vector
    pub fn spawn_missile(&mut self, enemy: Option<EnemyID>) {
        self.missiles.push(Missile::new(enemy));
    }
}
