use std::convert::From;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn rotate(&self, rot: Vec3) -> Self {
        let rm = self.rot_matrix(rot);
        Vec3::new(rm[0] * (*self), rm[1] * (*self), rm[2] * (*self))
    }

    fn rot_matrix(&self, rot: Vec3) -> [Self; 3] {
        let (sa, ca) = (rot.x.sin(), rot.x.cos());
        let (sb, cb) = (rot.y.sin(), rot.y.cos());
        let (sc, cc) = (rot.z.sin(), rot.z.cos());

        [
            Vec3::new(cb * cc, sa * sb * cc - ca * sc, ca * sb * cc + sa * sc),
            Vec3::new(cb * sc, sa * sb * sc + ca * cc, ca * sb * sc - sa * cc),
            Vec3::new(-sb, sa * cb, ca * cb),
        ]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(v: [f32; 3]) -> Self {
        Self::new(v[0], v[1], v[2])
    }
}

impl Into<[f32; 3]> for Vec3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}
