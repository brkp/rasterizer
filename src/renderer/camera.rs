use arche::Point;

#[derive(Default, Debug)]
pub struct Camera {
    pub pos: Point,
    pub rot: f32,
}

#[derive(Default, Debug)]
pub struct Viewport {
    pub w: f32,
    pub h: f32,
    pub d: f32,

    pub wc: f32,
    pub hc: f32,
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

    pub fn project(&self, v: [f32; 3]) -> Point {
        self.viewport_to_screen(
            (v[0] + 0.0) * self.d / (v[2] + 3.0),
            (v[1] - 0.0) * self.d / (v[2] + 3.0),
        )
    }
}
