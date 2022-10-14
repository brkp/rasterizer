use arche::prelude::*;
use arche::winit::event::VirtualKeyCode;
use arche::winit_input_helper::WinitInputHelper;

use crate::renderer::scene::Scene;
use crate::renderer::vec3::Vec3;

pub struct MainState {
    scene: Scene,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Box<dyn State> {
        Box::new(MainState {
            scene: Scene::new(ctx),
        })
    }
}

impl State for MainState {
    fn on_start(&mut self, ctx: &mut Context) {
        self.scene
            .push_object(obj::Obj::load("assets/diablo.obj").unwrap());
    }

    fn handle_events(&mut self, ctx: &mut Context, input: &WinitInputHelper) -> Trans {
        if input.quit() || input.key_pressed(VirtualKeyCode::Escape) {
            return Trans::Quit;
        }

        let mut pos_diff = Vec3::default();
        let mut rot_diff = Vec3::default();

        if input.key_held(VirtualKeyCode::W) { pos_diff.z += 0.001; }
        if input.key_held(VirtualKeyCode::S) { pos_diff.z -= 0.001; }
        if input.key_held(VirtualKeyCode::A) { pos_diff.x -= 0.001; }
        if input.key_held(VirtualKeyCode::D) { pos_diff.x += 0.001; }
        if input.key_held(VirtualKeyCode::E) { pos_diff.y -= 0.001; }
        if input.key_held(VirtualKeyCode::Q) { pos_diff.y += 0.001; }
        if input.key_held(VirtualKeyCode::R) { self.scene.camera.rot = Vec3::default() }

        if input.key_held(VirtualKeyCode::Left)  { rot_diff.y -= 0.001 };
        if input.key_held(VirtualKeyCode::Right) { rot_diff.y += 0.001 };

        self.scene.camera.rotate(rot_diff);
        self.scene.camera.move_pos(pos_diff);

        Trans::None
    }

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        draw::fill(ctx, Color::rgb(0x6495ed));

        self.scene.render(ctx);
    }
}
