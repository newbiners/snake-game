use ggez::GameResult;

pub struct Food {
    pub position: (i32, i32),
}

impl Food {
    pub fn new() -> Self {
        Self { position: (15, 15) }
    }

    pub fn respawn(&mut self) {
        self.position = (
            rand::random::<i32>().abs() % 20,
            rand::random::<i32>().abs() % 20,
        );
    }

    pub fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        assets: &crate::assets::Assets,
    ) -> GameResult {
        use crate::utils::grid_to_pixel;
        use ggez::graphics::DrawParam;

        let pos = grid_to_pixel(self.position);
        canvas.draw(&assets.apple, DrawParam::default().dest([pos.0, pos.1]));
        Ok(())
    }
}
