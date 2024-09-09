

pub struct Matrix3x3 {
    data : [f32; 9]
}

impl Matrix3x3 {
    pub fn data(&self) -> [f32; 9] {
        self.data
    }   

    pub fn identity() -> Matrix3x3 {
        Matrix3x3{
            data : [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            ]
        }
    }

    pub fn new(c11 : f32, c12 : f32, c13 : f32, c21 : f32, c22 : f32, c23 : f32, c31 : f32, c32 : f32, c33 : f32) -> Matrix3x3 {
        Matrix3x3{
            data : [
                c11, c12, c13,
                c21, c22, c23,
                c31, c32, c33
            ]
        }
    }
}