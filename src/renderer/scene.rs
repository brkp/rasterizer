use super::camera::{Camera, Viewport};
use super::vec3::Vec3;
use arche::{draw, pt, Color, Context, Point};

use std::f32::consts::PI;

pub struct Scene {
    pub camera: Camera,
    pub viewport: Viewport,
    pub objects: Vec<obj::Obj>,
}

impl Scene {
    pub fn new(ctx: &Context) -> Self {
        Self {
            camera: Camera::new(
                Vec3::new(0.0, 0.0, -3.0),
                Vec3::new(0.0, 0.0, 0.0),
                // Vec3::default()
            ),
            viewport: Viewport::new(
                4.0 / 3.0,
                1.0,
                1.0,
                ctx.config.width as f32,
                ctx.config.height as f32,
            ),
            objects: Vec::new(),
        }
    }

    pub fn push_object(&mut self, object: obj::Obj) {
        self.objects.push(object);
    }

    pub fn render(&self, ctx: &mut Context) {
        for obj_file in self.objects.iter() {
            obj_file.data.objects.iter().for_each(|obj| {
                obj.groups.iter().for_each(|group| {
                    group.polys.iter().for_each(|tri| {
                        let v0: Vec3 = obj_file.data.position[tri.0[0].0].into();
                        let v1: Vec3 = obj_file.data.position[tri.0[1].0].into();
                        let v2: Vec3 = obj_file.data.position[tri.0[2].0].into();

                        draw::triangle(
                            ctx,
                            self.viewport.project(self.camera.relative(v0)),
                            self.viewport.project(self.camera.relative(v1)),
                            self.viewport.project(self.camera.relative(v2)),
                            Color::default(),
                        )
                    });
                })
            });
        }
    }
}
