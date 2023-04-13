use super::consts::VEC_FRONT;
use super::consts::VEC_RIGHT;
use super::consts::VEC_UP;
use super::texture::Texture;
use super::transformation::Transformation;
use super::vertex::Vertex;
use crate::c_str;
use crate::cg::shader::Shader;
use crate::game::drawable::Drawable;
use crate::game::flight::steerable::Steerable;
use crate::offset_of;
use cgmath::prelude::*;
use cgmath::Deg;
use cgmath::Quaternion;
use cgmath::{vec2, vec3};
use gl;
use image;
use image::DynamicImage::*;
use image::GenericImage;
use log::error;
use log::warn;
use std::ffi::CStr;
use std::ffi::{CString, OsStr};
use std::mem::size_of;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;
use tobj;

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
    pub transformation: Transformation,
    pub orientation: Quaternion<f32>,
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
            transformation: Transformation::default(),
            orientation: Quaternion::from_angle_x(Deg(0.)),
            directory: String::new(),
        }
    }
}

//self.transformation.yaw += amount;
// self.right = (rotation * self.right).normalize();
// self.front = (rotation * self.front).normalize();
//self.model_matrix = self.model_matrix * Matrix4::from(rotation);

impl Steerable for Model {
    fn pitch(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(*VEC_RIGHT, Deg(amount));
        self.orientation = self.orientation * rotation;
    }

    fn yaw(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(*VEC_UP, Deg(amount));
        self.orientation = self.orientation * rotation;
    }

    fn roll(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(*VEC_FRONT, Deg(amount));
        self.orientation = self.orientation * rotation;
    }

    fn forward(&mut self, throttle: f32) {
        self.transformation.translation += self.front() * throttle;
    }
}

impl Drawable for Model {
    unsafe fn draw(&self, shader: &Shader) {
        if self.directory.is_empty() {
            error!(
                "Attempt to draw a model that was not loaded. Use the `load_model` method first."
            );
            panic!("Attempt to draw a model that was not loaded");
        }

        let matrix = self.build_model_matrix();
        shader.set_mat4(c_str!("model"), &matrix);

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
    /// Load a ready to draw model from an `.obj` file
    pub fn new(path: &str) -> Model {
        let mut model = Model::default();
        model.load_model(path);
        unsafe { model.setup_mesh() }
        model
    }

    /// Construct a model matrix based on model's orientation, scale and translation
    fn build_model_matrix(&self) -> Matrix4 {
        let m = Matrix4::identity();
        let s = m * Matrix4::from_scale(self.transformation.scale);
        let t = m * Matrix4::from_translation(self.transformation.translation);
        let r = m * Matrix4::from(self.orientation);
        // Why is the order like this?
        // Those operations apply right-to-left, so first, when the model
        // is at [0,0,0], we rotate, then translate to the desired point
        // and only then we scale. Messing up this order results in
        // unexpected results like the model rotating around world origin
        // instead of its own local axis
        s * t * r
    }

    /// Get the model's position in world coordinates
    pub fn position(&self) -> Point3 {
        let m = self.build_model_matrix();
        Point3::from_vec(vec3(
            m.w.x,
            m.w.y,
            m.w.z,
        ))
    }

    pub fn front(&self) -> Vector3 {
        self.orientation.rotate_vector(*VEC_FRONT).normalize()
    }

    /// Scale the model based on its current scale.
    /// For example: scaling by 0.5 and then by 2.0 restores original size
    pub fn scale(&mut self, scale: f32) -> &mut Self {
        self.transformation.scale *= scale;
        self
    }

    /// Set the scale of the model
    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        self.transformation.scale = scale;
        self
    }

    pub fn translate(&mut self, t: Vector3) -> &mut Self {
        self.transformation.translation = t;
        self
    }

    pub fn rotate(&mut self, axis: Vector3, angle: Deg<f32>) -> &mut Self {
        self.orientation = self.orientation * Quaternion::from_axis_angle(axis, angle);
        self
    }

    /// Use this cautiously!
    pub fn apply_quaternion(&mut self, quaternion: Quaternion<f32>) {
        warn!("Use of Model::apply_quaternion");
        self.orientation = self.orientation * quaternion;
    }

    // pub fn model_matrix(&self) -> Matrix4 {
    //     self.model_matrix
    // }

    // /// Use this cautiously!
    // pub fn set_model_matrix(&mut self, m: Matrix4) {
    //     self.model_matrix = m
    // }

    /// Load a model from file and store the resulting meshes in the meshes vector.
    pub fn load_model(&mut self, path: &str) {
        let path = Path::new(&path);

        if !path.exists() {
            error!("Attempt to load model from non-existent path: {path:?}");
            panic!();
        }

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
