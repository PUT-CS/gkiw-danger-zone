use cgmath::{Matrix3, Matrix4};

pub trait MatrixFmt {
    fn fmt(&self) -> String;
}

impl MatrixFmt for Matrix4<f32> {
    fn fmt(&self) -> String {
        "\n".to_string() + &[self.x, self.y, self.z, self.w]
            .map(|r| format!("{r:?}"))
            .map(|s| s.split_once(" ").unwrap().1.to_string())
            .join("\n")
    }
}

impl MatrixFmt for Matrix3<f32> {
    fn fmt(&self) -> String {
        "\n".to_string() + &[self.x, self.y, self.z]
            .map(|r| format!("{r:?}"))
            .map(|s| s.split_once(" ").unwrap().1.to_string())
            .join("\n")
    }
}
