use crate::renderer::vec3::Vec3;
use arche::Point;

#[derive(Default, Debug)]
pub struct Camera {
    pub pos: Vec3,
    pub rot: Vec3,
}

#[derive(Default, Debug)]
pub struct Viewport {
    pub w: f32,
    pub h: f32,
    pub d: f32,

    pub wc: f32,
    pub hc: f32,
}

impl Camera {
    pub fn new(pos: Vec3, rot: Vec3) -> Self {
        Self { pos, rot }
    }

    pub fn relative(&self, v: Vec3) -> Vec3 {
        v
    }
}

impl Viewport {
    pub fn new(w: f32, h: f32, d: f32, wc: f32, hc: f32) -> Self {
        Self { w, h, d, wc, hc }
    }

    pub fn viewport_to_screen(&self, x: f32, y: f32) -> Point {
        Point::new(
            (self.wc / 2.0 + x * self.wc / self.w) as i32,
            (self.hc / 2.0 - y * self.hc / self.h) as i32,
        )
    }

    pub fn project(&self, v: Vec3) -> Point {
        self.viewport_to_screen(
            (v.x + 0.0) * self.d / (v.z + 9.0),
            (v.y - 1.5) * self.d / (v.z + 9.0),
        )
    }
}
