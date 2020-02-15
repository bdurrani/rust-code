use std::fmt;
use std::ops;
use std::ops::{Add, Div, Neg, Sub};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2])
    }

    pub fn squared_length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2])
    }

    pub fn make_unit_vector(&mut self) {
        let k: f32 = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn unit_vector(vector: Vec3) -> Vec3 {
        vector / vector.length()
    }
}

impl fmt::Display for Vec3 {
    // https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(self.e[0], self.e[1], self.e[2])
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] / rhs.e[0],
            self.e[1] / rhs.e[1],
            self.e[2] / rhs.e[2],
        )
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn equality_works() {
        assert_eq!(
            Vec3::new(1_f32, 2_f32, 3_f32),
            Vec3::new(1_f32, 2_f32, 3_f32)
        );
    }

    #[test]
    fn addition_works() {
        let expected = Vec3::new(5_f32, 4_f32, 3_f32);
        let actual = Vec3::new(2_f32, 1_f32, 2_f32) + Vec3::new(3_f32, 3_f32, 1_f32);
        assert_eq!(expected, actual);
    }

    #[test]
    fn subtraction_works() {
        let expected = Vec3::new(-1_f32, -2_f32, 1_f32);
        let actual = Vec3::new(2_f32, 1_f32, 2_f32) - Vec3::new(3_f32, 3_f32, 1_f32);
        assert_eq!(expected, actual);
    }

    #[test]
    fn division_works() {
        let expected = Vec3::new(1_f32, 2_f32, 3_f32);
        let actual = Vec3::new(5_f32, 10_f32, 15_f32) / Vec3::new(5_f32, 5_f32, 5_f32);
        assert_eq!(expected, actual);
    }

    #[test]
    fn division_by_float_works() {
        let expected = Vec3::new(1_f32, 2_f32, 3_f32);
        let actual = Vec3::new(5_f32, 10_f32, 15_f32) / 5_f32;
        assert_eq!(expected, actual);
    }
}
