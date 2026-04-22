use ggez::{Context, GameResult};

pub struct Food {
    pub position: (i32, i32),
}

impl Food {
    pub fn new() -> Self {
        Self {
            position: (15, 15),
        }
    }

    pub fn respawn(&mut self) {
        self.position = (rand::random::<i32>().abs() % 20, rand::random::<i32>().abs() % 20);
    }

    pub fn draw(&self, canvas: &mut ggez::graphics::Canvas, ctx: &ggez::Context) -> GameResult {
        use ggez::graphics::{self, Color, DrawParam, Mesh};
        use crate::utils::grid_to_pixel;

        let pos = grid_to_pixel(self.position);
        let mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(pos.0, pos.1, 20.0, 20.0),
            Color::RED,
        )?;
        canvas.draw(&mesh, DrawParam::default());
        Ok(())
    }
}
