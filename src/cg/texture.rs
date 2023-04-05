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
