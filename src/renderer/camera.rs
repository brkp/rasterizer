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
        (v - self.pos).rotate(self.rot * -1.0)
    }

    pub fn move_pos(&mut self, diff: Vec3) {
        self.pos = self.pos + diff.rotate(Vec3::new(0.0, self.rot.y, self.rot.z));
    }

    pub fn rotate(&mut self, diff: Vec3) {
        self.rot = self.rot + diff.rotate(Vec3::new(0.0, self.rot.y, self.rot.z));
    }
}

impl Viewport {
    pub fn new(w: f32, h: f32, d: f32, wc: f32, hc: f32) -> Self {
        Self { w, h, d, wc, hc }
    }

    pub fn viewport_to_screen(&self, x: f32, y: f32) -> Point {
        Point::new(
            (self.wc / 2.0 + x * self.wc / self.w) as u32,
            (self.hc / 2.0 - y * self.hc / self.h) as u32,
        )
    }

    pub fn project(&self, v: Vec3) -> Point {
        self.viewport_to_screen(v.x * self.d / v.z, v.y * self.d / v.z)
    }
}
