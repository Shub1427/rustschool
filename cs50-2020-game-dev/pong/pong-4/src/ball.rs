use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh};
use ggez::{Context, GameResult};
use ggez::input::keyboard;
use ggez::nalgebra::{Point2};
use rand::Rng;

use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub struct Ball {
    ball: Mesh,
    init_pos: Point2<f32>,
    pos: Point2<f32>,
    dx: f32,
    dy: f32,
}

impl Ball {
    // Point2::new(WINDOW_WIDTH / 2.0 - (2.0 * dpi_factor), WINDOW_HEIGHT / 2.0 - (2.0 * dpi_factor))
    pub fn new(ctx: &mut Context, radius: f32, init_pos: Point2<f32>) -> GameResult<Self> {
        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;
        let mut rng = rand::thread_rng();
        let dx = if rng.gen_range(1, 2) as u8 == 1 { 200.0 } else { -200.0 };
        let dy: f32 = rng.gen_range(-100.0, 100.0);

        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::Fill(FillOptions::default()),
            Point2::new(0.0, 0.0),
            radius * dpi_factor,
            5.0,
            Color::from_rgb(255, 255, 255),
        )?;

        Ok(Ball {
            ball,
            pos: init_pos,
            init_pos,
            dx,
            dy
        })
    }

    fn random_dxdy(&mut self) -> (f32, f32) {
        self.pos = self.init_pos;
        let mut rng = rand::thread_rng();
        let dx = if rng.gen_range(1, 3) as u8 == 1 { 200.0 } else { -200.0 };
        let dy: f32 = rng.gen_range(-100.0, 100.0) * 1.5;
        (dx, dy)
    }

    pub fn update(&mut self, ctx: &mut Context, ellapsed_time: f32) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            let (dx, dy) = self.random_dxdy();
            self.dx = dx;
            self.dy = dy;
        } else {
            let new_x = self.pos.x + self.dx * ellapsed_time;
            let new_y = self.pos.y + self.dy * ellapsed_time;
            self.pos = Point2::new(new_x, new_y);
        }
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.ball, DrawParam::default().dest(self.pos))?;
        Ok(())
    }
}
