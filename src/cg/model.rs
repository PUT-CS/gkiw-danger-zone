use crate::cg::shader::Shader;
use crate::game::flight::steerable::Steerable;
use cgmath::prelude::*;
use cgmath::Deg;
use cgmath::Quaternion;
use cgmath::Vector2;
use cgmath::{vec2, vec3};
use gl;
use gl::DEBUG_CALLBACK_FUNCTION;
use image;
use image::DynamicImage::*;
use image::GenericImage;
use itertools::Position;
use log::info;
use log::warn;
use std::ffi::{CString, OsStr};
use std::mem::size_of;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;
use tobj;
use log::error;
use worldgen::constraint;
use worldgen::noise::perlin::PerlinNoise;
use worldgen::noisemap::NoiseMapGeneratorBase;
use worldgen::noisemap::{NoiseMap, NoiseMapGenerator, Seed, Size, Step};
use worldgen::world::tile::{Constraint, ConstraintType};
use worldgen::world::{Tile, World};
use crate::game::drawable::Drawable;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub tex_coords: Vector2<f32>,
    pub tangent: Vector3,
    pub bitangent: Vector3,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tex_coords: Vector2::zero(),
            tangent: Vector3::zero(),
            bitangent: Vector3::zero(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Texture {
    pub id: u32,
    pub type_: String,
    pub path: String,
}

impl Default for Texture {
    fn default() -> Self {
        Texture {
            id: 0,
            type_: "none".to_string(),
            path: "none".to_string(),
        }
    }
}

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

#[derive(Clone, Debug)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub vao: u32,
    vbo: u32,
    ebo: u32,
    pub textures_loaded: Vec<Texture>,
    directory: String,
    model_matrix: Matrix4,
    pub position: Point3,
    pub front: Vector3,
    pub up: Vector3,
    pub right: Vector3,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            vertices: vec![],
            indices: vec![],
            textures: vec![],
            vao: u32::MAX,
            vbo: u32::MAX,
            ebo: u32::MAX,
            textures_loaded: vec![],
            model_matrix: Matrix4::from_value(1.0),
            directory: String::new(),
            position: Point3::new(0., 0., 0.),
            front: Vector3::unit_z() * -1.,
            up: Vector3::unit_y(),
            right: Vector3::unit_x(),
        }
    }
}

impl Steerable for Model {
    fn pitch(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(self.right, Deg(amount));
        self.model_matrix = self.model_matrix * Matrix4::from(rotation);
    }

    fn yaw(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(self.up, Deg(amount));
        self.model_matrix = self.model_matrix * Matrix4::from(rotation);
    }

    fn roll(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(self.front, Deg(amount));
        self.model_matrix = self.model_matrix * Matrix4::from(rotation);
        //self.up = (rotation * self.up).normalize();
        //self.right = (rotation * self.right).normalize();
    }

    fn forward(&mut self, throttle: f32) {
        self.position += self.front * throttle;
        self.model_matrix = self.model_matrix * Matrix4::from_translation(self.front * throttle);
    }
}

impl Drawable for Model {
    unsafe fn draw(&self, shader: &Shader) {

        if self.directory == "" {
            error!("Attempt to draw a model that was not loaded. Use the `load_model` method first.");
            panic!("Attempt to draw a model that was not loaded");
        }
        
        // bind appropriate textures
        let mut diffuse_nr = 0;
        let mut specular_nr = 0;
        let mut normal_nr = 0;
        let mut height_nr = 0;
        for (i, texture) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);
            let name = &texture.type_;
            let number = match name.as_str() {
                "texture_diffuse" => {
                    diffuse_nr += 1;
                    diffuse_nr
                }
                "texture_specular" => {
                    specular_nr += 1;
                    specular_nr
                }
                "texture_normal" => {
                    normal_nr += 1;
                    normal_nr
                }
                "texture_height" => {
                    height_nr += 1;
                    height_nr
                }
                _ => panic!("unknown texture type"),
            };
            // set the sampler to the correct texture unit
            let sampler = CString::new(format!("{}{}", name, number)).unwrap();
            gl::Uniform1i(
                gl::GetUniformLocation(shader.id, sampler.as_ptr()),
                i as i32,
            );
            // bind the texture
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
        }

        // draw mesh
        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as i32,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
        gl::BindVertexArray(0);

        // always good practice to set everything back to defaults once configured.
        gl::ActiveTexture(gl::TEXTURE0);
    }

}

impl Model {
    pub fn new<T>(path: T) -> Model
    where
        T: ToString + AsRef<OsStr> + std::fmt::Display,
    {
        let mut model = Model::default();
        model.load_model(path);
        unsafe { model.setup_mesh() }
        model
    }

    pub fn model_matrix(&self) -> Matrix4 {
        self.model_matrix
    }

    // load a model from file and stores the resulting meshes in the meshes vector.
    pub fn load_model<T>(&mut self, path: T)
    where
        T: ToString + AsRef<OsStr>,
    {
        let path = Path::new(&path);

        self.directory = path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_str()
            .unwrap()
            .into();
        let obj = tobj::load_obj(path);
        let (models, materials) = obj.unwrap();
        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // data to fill
            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(Vertex {
                    position: vec3(p[i * 3], p[i * 3 + 1], p[i * 3 + 2]),
                    normal: vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2]),
                    tex_coords: vec2(t[i * 2], t[i * 2 + 1]),
                    ..Vertex::default()
                })
            }

            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                if !material.diffuse_texture.is_empty() {
                    let texture =
                        self.load_material_texture(&material.diffuse_texture, "texture_diffuse");
                    textures.push(texture);
                }
                if !material.specular_texture.is_empty() {
                    let texture =
                        self.load_material_texture(&material.specular_texture, "texture_specular");
                    textures.push(texture);
                }
                if !material.normal_texture.is_empty() {
                    let texture =
                        self.load_material_texture(&material.normal_texture, "texture_normal");
                    textures.push(texture);
                }
            }
            self.vertices = vertices;
            self.indices = indices;
            self.textures = textures;
        }
    }

    fn load_material_texture(&mut self, path: &str, type_name: &str) -> Texture {
        {
            let texture = self.textures_loaded.iter().find(|t| t.path == path);
            if let Some(texture) = texture {
                return texture.clone();
            }
        }

        let texture = Texture {
            id: unsafe { texture_from_file(path, &self.directory) },
            type_: type_name.into(),
            path: path.into(),
        };
        self.textures_loaded.push(texture.clone());
        texture
    }

    pub fn reload_mesh(&mut self) {
        unsafe { self.setup_mesh() };
    }

    unsafe fn setup_mesh(&mut self) {
        // create buffers/arrays
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);
        gl::GenBuffers(1, &mut self.ebo);

        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        let size = (self.indices.len() * size_of::<u32>()) as isize;
        let data = &self.indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        let size = size_of::<Vertex>() as i32;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, position) as *const c_void,
        );
        // vertex normals
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, normal) as *const c_void,
        );
        // vertex texture coords
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, tex_coords) as *const c_void,
        );
        // vertex tangent
        gl::EnableVertexAttribArray(3);
        gl::VertexAttribPointer(
            3,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, tangent) as *const c_void,
        );
        // vertex bitangent
        gl::EnableVertexAttribArray(4);
        gl::VertexAttribPointer(
            4,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, bitangent) as *const c_void,
        );

        gl::BindVertexArray(0);
    }

    pub fn randomize_height(&mut self) {
        warn!("Randomizing height");
        let noise = PerlinNoise::new();

        let nm1 = NoiseMap::new(noise)
            .set(Seed::of("H?"))
            .set(Step::of(0.005, 0.005));

        let nm2 = NoiseMap::new(noise)
            .set(Seed::of("dasdasdada"))
            .set(Step::of(0.05, 0.05));

        let nm = Box::new(nm1 + nm2 * 4);
        let chunk = nm.generate_sized_chunk(Size::of(100, 100), 0, 0);
        let mut idx = 0;
        for row in chunk {
            for number in row {
                self.vertices[idx].position.y += number as f32 / 10.;
                idx += 1;
            }
        }
    }
}

unsafe fn texture_from_file(path: &str, directory: &str) -> u32 {
    let filename = format!("{}/{}", directory, path);
    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);
    let img = image::open(&Path::new(&filename)).expect("Texture failed to load");
    let img = img.flipv();
    let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
    };

    let data = img.raw_pixels();

    gl::BindTexture(gl::TEXTURE_2D, texture_id);
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        format as i32,
        img.width() as i32,
        img.height() as i32,
        0,
        format,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR as i32,
    );
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    texture_id
}

fn round(number: f32, rounding: i32) -> f32 {
    let scale: f32 = 10_f64.powi(rounding) as f32;
    (number * scale).round() / scale
}
