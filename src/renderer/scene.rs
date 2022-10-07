use super::camera::{Camera, Viewport};
use arche::{draw, pt, Color, Context, Point};

pub struct Scene {
    camera: Camera,
    viewport: Viewport,
    objects: Vec<obj::Obj>,
}

impl Scene {
    pub fn new(ctx: &Context) -> Self {
        Self {
            camera: Camera::default(),
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
                        let v0 = obj_file.data.position[tri.0[0].0];
                        let v1 = obj_file.data.position[tri.0[1].0];
                        let v2 = obj_file.data.position[tri.0[2].0];

                        draw::triangle(
                            ctx,
                            self.viewport.project(v0.into()),
                            self.viewport.project(v1.into()),
                            self.viewport.project(v2.into()),
                            Color::default(),
                        )
                    });
                })
            });
        }
    }
}
