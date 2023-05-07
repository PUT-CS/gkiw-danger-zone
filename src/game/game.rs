use crate::audio::audio::Audio;
use crate::audio::audio_manager::{AudioManager, SoundEffect, SOUNDS};
use crate::audio::messages::AudioMessage;
use crate::cg::light::{DirectionalLight, PointLight};
use crate::game::targeting_data::{self, TargetingData};
use crate::{DELTA_TIME, GLFW_TIME, SCR_HEIGHT, SCR_WIDTH};
use glfw::ffi::glfwSwapInterval;
use glfw::{Context, Glfw, Window, WindowEvent};
use itertools::Itertools;
use log::{error, info, warn};
use rayon::ThreadPoolBuilder;
use std::sync::mpsc::{self, Receiver, Sender};
extern crate glfw;
use self::glfw::{Action, Key};
use super::enemies::Enemies;
use super::hud::hud::Hud;
use super::missile::{EnemyID, MissileMessage};
use super::missile_guidance::GuidanceStatus;
use super::modeled::Modeled;
use super::particle_generation::ParticleGeneration;
use super::terrain::Terrain;
use super::{missile::Missile, player::Player};
use crate::c_str;
use crate::cg::camera::Movement;
use crate::cg::model::Model;
use crate::cg::shader::Shader;
use crate::game::drawable::Drawable;
use crate::game::id_gen::IDGenerator;
use crate::key_pressed;
use cgmath::ortho;
use cgmath::{vec3, Deg, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};
use lazy_static::lazy_static;
use std::ffi::CStr;
use std::sync::Mutex;

pub const TARGET_ENEMIES: usize = 3;
pub const MISSILE_COOLDOWN: f64 = 0.5;
pub const SWITCH_COOLDOWN: f64 = 0.5;

lazy_static! {
    pub static ref ID_GENERATOR: Mutex<IDGenerator> = Mutex::new(IDGenerator::default());
}

pub struct Game {
    player: Player,
    enemies: Enemies,
    missiles: Vec<Missile>,
    terrain: Terrain,
    skybox: Model,
    last_launch_time: f64,
    last_target_switch_time: f64,
    targeting_data: Option<TargetingData>,
    pub glfw: Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
    audio: Audio,
    hud: Hud,
    directional_light: DirectionalLight,
    point_light: PointLight,
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

        let (tx, rx) = mpsc::channel::<AudioMessage>();
        let audio = Audio::new(tx);
        rayon::spawn(move || {
            AudioManager::run(rx);
        });

        let mut terrain = Terrain::default();
        terrain
            .model
            .set_scale(0.05)
            .set_translation(vec3(0.0, -150., 0.0));

        let mut player = Player::default();
        audio.play(SoundEffect::CockpitAmbient, true);

        player
            .cockpit_mut()
            .set_translation(vec3(0.0, -0.3, 0.0))
            .set_scale(0.5)
            .rotate(Vector3::unit_y(), Deg(-90.));

        let enemies = Enemies::default();

        let mut skybox = Model::new("resources/objects/skybox/skybox.obj");
        skybox.set_scale(1000.);

        let directional_light = DirectionalLight::new(Vector3::new(-0.2, -1., -0.3));

        let point_light = PointLight::new(Point3::new(0.5, 0.0, 0.5));

        let hud = Hud::new();

        let targeting_data = None;

        Game {
            player,
            enemies,
            missiles: vec![],
            terrain,
            skybox,
            last_target_switch_time: glfw.get_time() - SWITCH_COOLDOWN,
            last_launch_time: glfw.get_time() - MISSILE_COOLDOWN,
            targeting_data,
            glfw,
            window,
            events,
            audio,
            hud,
            directional_light,
            point_light,
        }
    }

    /// Compute new positions of all game objects based on input and state of the game
    pub fn update(&mut self) {
	//dbg!(&self.player.aircraft().model().position());
	//dbg!(&self.player.camera().position());
        self.player.apply_controls();
        self.player.aircraft_mut().apply_decay();
        self.respawn_enemies();
        self.enemies.map.values_mut().for_each(|e| {
            let position = e.aircraft().model().position();
            let front = e.aircraft().model().front();
            e.aircraft_mut()
                .particle_generator_mut()
                .update_particles(position, 1, front);
            let delta = unsafe { DELTA_TIME };

            // e.aircraft_mut().model_mut().forward(50. * delta);
            // e.aircraft_mut().model_mut().pitch(50. * delta);
            // e.aircraft_mut().model_mut().roll(50. * delta);
        });
        let shot_down = self.update_missiles();
        self.enemies.map.retain(|id, _| !shot_down.contains(id));
        self.missiles.iter_mut().for_each(|m| {
            let position = m.model().position();
            let front = m.model().front();
            m.particle_generator_mut()
                .update_particles(position, 1, front);
        });
        self.missiles
            .retain(|m| !matches!(m.guidance, GuidanceStatus::None(0)));
        self.player.aircraft_mut().guns_mut().update();
        if let Some(hit_enemies) = self
            .player
            .aircraft()
            .guns()
            .check_collisions(&self.enemies)
        {
            self.enemies.map.retain(|id, _| !hit_enemies.contains(id));
            self.targeting_data = None;
        }
        self.update_targeting();
        self.hud
            .update(&self.player.camera(), &self.enemies, &self.targeting_data);
    }

    /// Make the Enemies struct check for missing enemies and respawn them
    pub fn respawn_enemies(&mut self) {
        self.enemies.respawn_enemies();
    }

    /// If there's an enemy being targeted, countdown the lock time
    fn update_targeting(&mut self) {
        match &mut self.targeting_data {
            // no lock yet
            Some(data) if data.left_until_lock > 0. => {
                data.left_until_lock -= unsafe { DELTA_TIME as f64 };
            }
            _ => {}
        }
        if let Some(data) = &self.targeting_data {
            if !self
                .player
                .targetable_enemies(&self.enemies)
                .unwrap_or_else(|| vec![])
                .contains(&data.target_id)
            {
                warn!("Target lost");
                self.targeting_data = None;
            }
        }
    }

    pub unsafe fn draw(&mut self, shader: &Shader, hud_shader: &Shader) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        shader.use_program();
        shader.set_int(c_str!("material.diffuse"), 0);
        shader.set_int(c_str!("material.specular"), 1);
        //set light position and properties
        shader.set_vector3(c_str!("viewPos"), &self.player.camera().position().to_vec());
        shader.set_vector3(
            c_str!("dirLight.direction"),
            &self.directional_light.direction,
        );
        shader.set_vector3(c_str!("dirLight.ambient"), &self.directional_light.ambient);
        shader.set_vector3(c_str!("dirLight.diffuse"), &self.directional_light.diffuse);
        shader.set_vector3(
            c_str!("dirLight.specular"),
            &self.directional_light.specular,
        );
        shader.set_int(c_str!("material.diffuse"), 0);
        shader.set_int(c_str!("material.specular"), 1);
        //point light
        let light_position =
            self.player.camera().position.to_vec() + self.point_light.position.to_vec();
        shader.set_vector3(c_str!("pointLight.position"), &light_position);

        shader.set_float(c_str!("pointLight.constant"), self.point_light.constant);
        shader.set_float(c_str!("pointLight.linear"), self.point_light.linear);
        shader.set_float(c_str!("pointLight.quadratic"), self.point_light.quadratic);

        shader.set_vector3(c_str!("pointLight.ambient"), &self.point_light.ambient);
        shader.set_vector3(c_str!("pointLight.diffuse"), &self.point_light.diffuse);
        shader.set_vector3(c_str!("pointLight.specular"), &self.point_light.specular);

        shader.set_mat4(
            c_str!("projection"),
            &self.player.camera().projection_matrix(),
        );
        shader.set_mat4(c_str!("view"), &self.player.camera().view_matrix());

        // Drawing game objects starts here
        self.terrain.draw(&shader);
        self.skybox.draw(&shader);
        self.enemies.map.values_mut().for_each(|e| {
            e.aircraft.draw(shader);

            e.aircraft_mut().draw_particles(shader);
        });
        self.missiles.iter_mut().for_each(|m| {
            m.draw(shader);

            m.draw_particles(shader);
        });
        self.player.aircraft().guns().draw(shader);

        let time = self.glfw.get_time() as f32 * 2.0;
        self.player.cockpit_mut().set_translation(vec3(
            time.sin() * 0.003,
            time.cos().sin() * 0.003 - 0.31,
            time.cos() * 0.003,
        ));
        shader.set_mat4(c_str!("view"), &Matrix4::identity());
        self.player.cockpit.draw(&shader);

	hud_shader.use_program();
        self.hud.draw(hud_shader);
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
        key_pressed!(self.window, Key::Escape, self.window.set_should_close(true));
        key_pressed!(
            self.window,
            Key::W,
            self.player.process_key(Movement::PitchDown)
        );
        key_pressed!(
            self.window,
            Key::S,
            self.player.process_key(Movement::PitchUp)
        );
        key_pressed!(
            self.window,
            Key::A,
            self.player.process_key(Movement::RollLeft)
        );
        key_pressed!(
            self.window,
            Key::D,
            self.player.process_key(Movement::RollRight)
        );
        key_pressed!(
            self.window,
            Key::E,
            self.player.process_key(Movement::YawRight)
        );
        key_pressed!(
            self.window,
            Key::Q,
            self.player.process_key(Movement::YawLeft)
        );
        key_pressed!(
            self.window,
            Key::LeftShift,
            self.player.process_key(Movement::ThrottleUp)
        );
        key_pressed!(
            self.window,
            Key::LeftControl,
            self.player.process_key(Movement::ThrottleDown)
        );
        key_pressed!(self.window, Key::M, {
            if !self.player.aircraft().guns().firing {
                self.player.guns_sound = self.audio.play(SoundEffect::Guns, true);
            }
            self.fire_guns();
        });
        if self.window.get_key(Key::M) == Action::Release {
            if self.player.aircraft().guns().firing {
                self.audio.stop(self.player.guns_sound);
                self.player.aircraft_mut().guns_mut().stop_firing();
            }
        }
        key_pressed!(self.window, Key::Space, self.launch_missile());
        key_pressed!(self.window, Key::K, self.switch_target());
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn switch_target(&mut self) {
        if self.last_target_switch_time + SWITCH_COOLDOWN > self.glfw.get_time() {
            return;
        }

        match &self.targeting_data {
            // If there's already a target, look for the new closest one
            Some(_) => {
                if let Some(new_id) = self.player.targeted_enemy_id_nth(&self.enemies, 0) {
                    self.targeting_data = Some(TargetingData::new(new_id));
                }
            }
            // if there's no target, look for one and if it's present, switch to it
            None => {
                if let Some(id) = self.player.targeted_enemy_id_nth(&self.enemies, 0) {
                    self.targeting_data = Some(TargetingData::new(id))
                }
            }
        }

        self.last_target_switch_time = self.glfw.get_time();
    }

    /// Perform all actions necessary to launch a missile.
    /// The game keeps track of the time of last missile launch
    /// and doesn't let the player do it again before a specified time has passed.
    /// Modify MISSILE_COOLDOWN to adjust.
    pub fn launch_missile(&mut self) {
        if self.last_launch_time + MISSILE_COOLDOWN > self.glfw.get_time() {
            return;
        }
        if let Some(data) = &self.targeting_data {
            // targeting, but no lock yet
            if data.left_until_lock > 0. {
                warn!("No lock");
                self.last_launch_time = unsafe { GLFW_TIME };
                return;
            }
            let enemy = self.enemies.get_by_id(data.target_id);
            let missile = Missile::new(self.player.camera(), enemy);
            self.missiles.push(missile);

            self.audio.play(SoundEffect::MissileLaunch, false);

            self.last_launch_time = unsafe { GLFW_TIME }
        }
    }

    /// Give the missiles a reference to the Enemy they are currently
    /// targeting so they can mutate their state accordingly.
    /// Returns a vector of IDs of shot down enemies
    pub fn update_missiles(&mut self) -> Vec<EnemyID> {
        let mut shot_down = Vec::with_capacity(self.missiles.len());
        self.missiles.iter_mut().for_each(|missile| {
            let enemy = missile
                .target()
                .and_then(|id| self.enemies.get_mut_by_id(id))
                .or_else(|| None);
            if let Some(MissileMessage::HitEnemy(id)) = missile.update(enemy.as_deref()) {
                shot_down.push(id);
                self.targeting_data = None;
            }
        });
        shot_down
    }

    pub fn fire_guns(&mut self) {
        let camera = self.player.camera().clone();
        self.player.aircraft_mut().fire_guns(&camera);
    }

    pub fn exit_hook(&mut self) {
        self.audio.exit_hook();
    }
}
