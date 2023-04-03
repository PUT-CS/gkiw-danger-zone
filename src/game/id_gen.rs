use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
pub enum IDKind {
    Enemy,
}

pub struct IDGenerator {
    ids: HashMap<IDKind, u32>,
}

impl Default for IDGenerator {
    fn default() -> Self {
        IDGenerator {
            ids: HashMap::new(),
        }
    }
}

impl IDGenerator {
    pub fn get_new_id_of(&mut self, kind: IDKind) -> u32 {
        match self.ids.get(&kind).cloned() {
            Some(id) => {
                self.ids.insert(kind, id + 1);
                id + 1
            }
            None => {
                self.ids.insert(kind, 0);
                0
            }
        }
    }
}
