#[allow(dead_code)]
pub fn interp(x1: f32, x2: f32, t: f32) -> f32 {
    x1 + (x2 - x1) * t
}

#[allow(dead_code)]
pub fn cmid(x: i32, min: i32, max: i32) -> i32 {
    if x < min {
        min
    } else {
        if x > max {
            max
        } else {
            x
        }
    }
}

pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[allow(dead_code)]
impl Vector4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4f {
        Vector4f { x, y, z, w }
    }

    // |v|
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    // z = x + y
    fn add(&mut self, x: Vector4f, y: Vector4f) {
        self.x = x.x + y.x;
        self.y = x.y + y.y;
        self.z = x.z + y.z;
        self.w = 1.0;
    }

    // z = x - y
    fn sub(&mut self, x: Vector4f, y: Vector4f) {
        self.x = x.x - y.x;
        self.y = x.y - y.y;
        self.z = x.z - y.z;
        self.w = 1.0;
    }

    // dot product
    fn dotp(&mut self, x: Vector4f, y: Vector4f) -> f32 {
        let res = x.x * y.x + x.y * y.y + x.z * y.z;
        res
    }

    // cross product
    fn crossp(&mut self, x: Vector4f, y: Vector4f) {
        self.x = x.y * y.z - x.z * y.y;
        self.y = x.z * y.x - x.x * y.z;
        self.z = x.x * y.y - x.y * y.x;
        self.w = 1.0;
    }

    // interpolate between two vectors
    fn interp(&mut self, x: Vector4f, y: Vector4f, t: f32) {
        self.x = interp(x.x, y.x, t);
        self.y = interp(x.y, y.y, t);
        self.z = interp(x.z, y.z, t);
        self.w = 1.0;
    }

    // normalize
    fn normalize(&mut self) {
        let len = self.length();
        if len != 0.0 {
            let inv = 1.0 / len;
            self.x *= inv;
            self.y *= inv;
            self.z *= inv;
        }
    }

    // matrix apply
    fn matrix_apply(&mut self, x: Vector4f, m: Matrix4f) {
        self.x = x.x * m.m[0][0] + x.y * m.m[1][0] + x.z * m.m[2][0] + x.w * m.m[3][0];
        self.y = x.x * m.m[0][1] + x.y * m.m[1][1] + x.z * m.m[2][1] + x.w * m.m[3][1];
        self.z = x.x * m.m[0][2] + x.y * m.m[1][2] + x.z * m.m[2][2] + x.w * m.m[3][2];
        self.w = x.x * m.m[0][3] + x.y * m.m[1][3] + x.z * m.m[2][3] + x.w * m.m[3][3];
    }
}

#[derive(Clone, Copy)]
pub struct Matrix4f {
    pub m: [[f32; 4]; 4],
}

#[allow(dead_code)]
impl Matrix4f {
    pub fn new() -> Matrix4f {
        Matrix4f {
            m: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }

    // c = a + b
    pub fn add(&mut self, a: Matrix4f, b: Matrix4f) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = a.m[i][j] + b.m[i][j];
            }
        }
    }

    // c = a - b
    pub fn sub(&mut self, a: Matrix4f, b: Matrix4f) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = a.m[i][j] - b.m[i][j];
            }
        }
    }

    // c = a * b
    pub fn mul(&mut self, a: Matrix4f, b: Matrix4f) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = a.m[i][0] * b.m[0][j]
                    + a.m[i][1] * b.m[1][j]
                    + a.m[i][2] * b.m[2][j]
                    + a.m[i][3] * b.m[3][j];
            }
        }
    }

    // c = a * f
    pub fn scale(&mut self, a: Matrix4f, f: f32) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = a.m[i][j] * f;
            }
        }
    }

    // identity matrix
    pub fn identity(&mut self) {
        self.m[0][0] = 1.0;
        self.m[0][1] = 0.0;
        self.m[0][2] = 0.0;
        self.m[0][3] = 0.0;
        self.m[1][0] = 0.0;
        self.m[1][1] = 1.0;
        self.m[1][2] = 0.0;
        self.m[1][3] = 0.0;
        self.m[2][0] = 0.0;
        self.m[2][1] = 0.0;
        self.m[2][2] = 1.0;
        self.m[2][3] = 0.0;
        self.m[3][0] = 0.0;
        self.m[3][1] = 0.0;
        self.m[3][2] = 0.0;
        self.m[3][3] = 1.0;
    }

    // set matrix zero
    pub fn zero(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = 0.0;
            }
        }
    }
}
