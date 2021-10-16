use super::*;

trait Renderable {
    fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        draw_2d: &Rc<geng::Draw2D>,
        camera: &Camera2d,
    );
}

impl Renderable for (&Circle, &Sprite) {
    fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        draw_2d: &Rc<geng::Draw2D>,
        camera: &Camera2d,
    ) {
        let mut aabb = AABB::point(self.0.position).extend_uniform(self.0.radius);
        if self.1.flipped {
            std::mem::swap(&mut aabb.x_min, &mut aabb.x_max);
        }
        draw_2d.textured_quad(framebuffer, camera, aabb, &self.1.texture, Color::WHITE);
    }
}

impl Renderable for (&Circle, &Health) {
    fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        draw_2d: &Rc<geng::Draw2D>,
        camera: &Camera2d,
    ) {
        let circle = self.0;
        let health = self.1;
        if health.fraction() == 1.0 {
            return;
        }

        let bar_position = circle.position + vec2(0.0, circle.radius);
        let bar_width = circle.radius;
        let bar_height = 1.0;
        let bar_aabb = AABB::point(bar_position)
            .extend_up(bar_height)
            .extend_symmetric(vec2(bar_width / 2.0, 0.0));
        draw_2d.quad(framebuffer, camera, bar_aabb, Color::rgb(0.0, 0.3, 0.0));
        let offset = bar_height * 0.2;
        let health_aabb = bar_aabb
            .extend_uniform(-offset)
            .extend_positive(vec2((health.fraction() - 1.0) * (bar_width - offset), 0.0));
        draw_2d.quad(framebuffer, camera, health_aabb, Color::rgb(0.0, 0.7, 0.0));
    }
}

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(constants::BACKGROUND_COLOR), None);
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);

        self.draw_game(framebuffer);
        self.draw_ui(framebuffer);
    }

    fn draw_game(&self, framebuffer: &mut ugli::Framebuffer) {
        // Draw particles
        for particle in &self.particles {
            self.geng.draw_2d().circle(
                framebuffer,
                &self.camera,
                particle.circle.position,
                particle.circle.radius,
                particle.color,
            );
        }

        let entities = self.skeletons.len() + self.knights.len();
        let mut sprites = Vec::with_capacity(entities + 1);
        let mut healths = Vec::with_capacity(entities);

        // Skeletons
        for skeleton in &self.skeletons {
            sprites.push((&skeleton.circle, &skeleton.sprite));
            healths.push((&skeleton.circle, &skeleton.health));
        }

        // Knights
        for knight in &self.knights {
            sprites.push((&knight.circle, &knight.sprite));
            // healths.push((&knight.circle, &knight.health));
        }

        // Player
        sprites.push((&self.player.circle, &self.player.sprite));

        for renderable in sprites {
            renderable.draw(framebuffer, self.geng.draw_2d(), &self.camera);
        }
        for renderable in healths {
            renderable.draw(framebuffer, self.geng.draw_2d(), &self.camera);
        }
    }

    fn draw_ui(&self, framebuffer: &mut ugli::Framebuffer) {
        let camera_view = camera_view(&self.camera, self.framebuffer_size);

        // Player health
        self.assets.font.draw(
            framebuffer,
            &self.camera,
            "Health",
            vec2(camera_view.x_min + 3.0, camera_view.y_max - 5.0),
            geng::TextAlign::LEFT,
            3.5,
            Color::WHITE,
        );

        // Draw player health
        let bar_position = vec2(camera_view.x_min + 3.0, camera_view.y_max - 8.0);
        let bar_width = 20.0;
        let bar_height = 2.0;
        let bar_aabb = AABB::point(bar_position).extend_positive(vec2(bar_width, bar_height));
        self.geng.draw_2d().quad(
            framebuffer,
            &self.camera,
            bar_aabb,
            Color::rgb(0.0, 0.3, 0.0),
        );
        let offset = 0.5;
        let health_aabb = bar_aabb.extend_uniform(-offset).extend_positive(vec2(
            (self.player.health.fraction() - 1.0) * (bar_width - offset),
            0.0,
        ));
        self.geng.draw_2d().quad(
            framebuffer,
            &self.camera,
            health_aabb,
            Color::rgb(0.0, 0.7, 0.0),
        );
    }
}

fn camera_view(camera: &geng::Camera2d, framebuffer_size: Vec2<f32>) -> AABB<f32> {
    let vertical_fov = camera.fov;
    let horizontal_fov = framebuffer_size.x * vertical_fov / framebuffer_size.y;
    AABB::ZERO.extend_symmetric(vec2(horizontal_fov, vertical_fov) / 2.0)
}
