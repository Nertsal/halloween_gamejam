use geng::Camera2d;

use super::*;

pub trait Renderable {
    fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        draw_2d: &Rc<geng::Draw2D>,
        camera: &Camera2d,
    );
}
