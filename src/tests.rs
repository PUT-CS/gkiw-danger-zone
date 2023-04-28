#[cfg(test)]
mod tests {

    extern crate glfw;
    use crate::cg::model::Model;
    use cgmath::{vec3, EuclideanSpace};

    #[test]
    fn model_position() {
        let mut model = Model::default();
        let translation = vec3(0.5, 1.0, 5.0);
        model.set_translation(translation);
        assert_eq!(model.position().to_vec(), translation);
    }
}
