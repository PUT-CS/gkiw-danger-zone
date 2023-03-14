use cg::{camera::Camera, model::Model, shader::Shader, terrain::Terrain, terrain::TerrainType};
use cgmath::{perspective, vec3, Deg, Matrix4, Vector3};
use game::player::{self, Player};
use game::flight::aircraft::AircraftKind::*;
use glfw::{ffi::glfwSwapInterval, Context};
use std::ffi::CStr;
use std::sync::mpsc::Receiver;
extern crate glfw;
use self::glfw::{Action, Key};
use crate::cg::camera::Movement;

const SCR_WIDTH: u32 = 1000;
const SCR_HEIGHT: u32 = 1000;

mod macros;
mod cg {
    pub mod camera;
    pub mod model;
    pub mod shader;
    pub mod terrain;
}
mod game {
    pub mod flight {
        pub mod aircraft;
        pub mod spec;
        pub mod steerable;
        pub mod control_surfaces;
    }
    pub mod player;
}

fn main() {
    let mut first_mouse = true;
    let mut last_x: f32 = SCR_WIDTH as f32 / 2.;
    let mut last_y: f32 = SCR_HEIGHT as f32 / 2.;

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.;

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
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

    let mut player = Player::new(Mig21);
    let mut camera = player.camera_mut();
    let shader = Shader::new("src/shaders/model.vs", "src/shaders/model.fs");
    let mig21 = Model::new("resources/objects/mig21/mig21.obj");
    let terrain = Terrain::new("resources/objects/terrain/terrain.obj", TerrainType::Desert);
    let skybox = Model::new("resources/objects/skybox/skybox.obj");

    while !window.should_close() {
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        process_events(
            &events,
            &mut first_mouse,
            &mut last_x,
            &mut last_y,
            &mut camera,
        );
        process_key(&mut window, delta_time, &mut camera);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            shader.use_program();

            // view/projection transformations
            let projection: Matrix4<f32> = perspective(
                Deg(camera.zoom),
                SCR_WIDTH as f32 / SCR_HEIGHT as f32,
                0.1,
                2000.0,
            );

            let view = camera.view_matrix();
            shader.set_mat4(c_str!("projection"), &projection);
            shader.set_mat4(c_str!("view"), &view);

            let mut model_matrix = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, -1.0));
            model_matrix = model_matrix * Matrix4::from_scale(glfw.get_time() as f32);
            model_matrix = model_matrix * Matrix4::from_axis_angle(Vector3::unit_x(), Deg(0.));
            shader.set_mat4(c_str!("model"), &model_matrix);
            mig21.draw(&shader);

            let mut model_matrix = Matrix4::<f32>::from_translation(vec3(0.0, -10.0, 0.0));
            model_matrix = model_matrix * Matrix4::from_scale(1000.0);
            shader.set_mat4(c_str!("model"), &model_matrix);
            terrain.draw(&shader);

            let mut model_matrix = Matrix4::<f32>::from_translation(vec3(0.0, -10.0, 0.0));
            model_matrix = model_matrix * Matrix4::from_scale(1000.0);
            shader.set_mat4(c_str!("model"), &model_matrix);
            skybox.draw(&shader);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}


pub fn process_events(
    events: &Receiver<(f64, glfw::WindowEvent)>,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    camera: &mut Camera,
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

                camera.process_mouse_movement(xoffset, yoffset);
            }
            glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                camera.process_mouse_scroll(yoffset as f32);
            }
            _ => {}
        }
    }
}

macro_rules! key_pressed {
    ($window:expr, $key:expr, $action:expr) => {
        if $window.get_key($key) == Action::Press {
            $action
        }
    };
}

pub fn process_key(window: &mut glfw::Window, delta_time: f32, camera: &mut Camera) {
    key_pressed!(window, Key::Escape, window.set_should_close(true));
    key_pressed!(
        window,
        Key::W,
        camera.process_keyboard_input(Movement::Forward, delta_time)
    );
    key_pressed!(
        window,
        Key::S,
        camera.process_keyboard_input(Movement::Backward, delta_time)
    );
    key_pressed!(
        window,
        Key::A,
        camera.process_keyboard_input(Movement::Left, delta_time)
    );
    key_pressed!(
        window,
        Key::D,
        camera.process_keyboard_input(Movement::Right, delta_time)
    );
    key_pressed!(
        window,
        Key::E,
        camera.process_keyboard_input(Movement::RollRight, delta_time)
    );
    key_pressed!(
        window,
        Key::Q,
        camera.process_keyboard_input(Movement::RollLeft, delta_time)
    );
}

