use super::camera::{Camera, Viewport};
use super::vec3::Vec3;
use crate::rand;
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

    fn project(&self, v: (Vec3, Vec3, Vec3)) -> Option<(Point, Point, Point)> {
        let v0 = self.camera.relative(v.0);
        let v1 = self.camera.relative(v.1);
        let v2 = self.camera.relative(v.2);

        for nv in [&v0, &v1, &v2] {
            if nv.z < 0.0 {
                return None;
            }
        }

        Some((
            self.viewport.project(v0),
            self.viewport.project(v1),
            self.viewport.project(v2),
        ))
    }

    pub fn render(&self, ctx: &mut Context) {
        self.objects.iter().for_each(|obj_file| {
            obj_file.data.objects.iter().for_each(|obj| {
                obj.groups.iter().for_each(|group| {
                    group.polys.iter().for_each(|tri| {
                        let verticies = (
                            obj_file.data.position[tri.0[0].0].into(),
                            obj_file.data.position[tri.0[1].0].into(),
                            obj_file.data.position[tri.0[2].0].into(),
                        );

                        if let Some((v0, v1, v2)) = self.project(verticies) {
                            draw::triangle(ctx, v0, v1, v2, Color::rgb(0xffffff));
                        }
                    });
                })
            });
        });
    }
}
