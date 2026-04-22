use ggez::GameResult;

pub struct Wall {
    pub segments: Vec<Vec<(i32, i32)>>, // 4 wall segments, each with multiple blocks
}

impl Wall {
    pub fn new() -> Self {
        let mut wall = Self {
            segments: Vec::new(),
        };
        wall.respawn();
        wall
    }

    /// Generate 4 random wall segments at random positions.
    /// Each segment is 3 blocks long, either horizontal or vertical.
    pub fn respawn(&mut self) {
        self.segments.clear();

        for _ in 0..4 {
            self.segments.push(Self::random_segment());
        }
    }

    /// Create a single random wall segment (3 blocks, horizontal or vertical).
    fn random_segment() -> Vec<(i32, i32)> {
        let x = rand::random::<i32>().abs() % 18 + 1; // 1..18 (avoid grid edge)
        let y = rand::random::<i32>().abs() % 18 + 1;
        let horizontal = rand::random::<bool>();

        let mut blocks = Vec::new();
        for i in 0..3 {
            if horizontal {
                blocks.push((x + i, y));
            } else {
                blocks.push((x, y + i));
            }
        }
        blocks
    }

    /// Check if a given position collides with any wall block.
    pub fn collides_with(&self, pos: (i32, i32)) -> bool {
        self.segments.iter().any(|seg| seg.contains(&pos))
    }

    pub fn draw(&self, canvas: &mut ggez::graphics::Canvas, ctx: &ggez::Context) -> GameResult {
        use crate::utils::grid_to_pixel;
        use ggez::graphics::{self, Color, DrawParam, Mesh};

        for segment in &self.segments {
            for &block in segment {
                let pos = grid_to_pixel(block);
                let mesh = Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(pos.0, pos.1, 20.0, 20.0),
                    Color::new(0.6, 0.6, 0.6, 1.0), // Abu-abu
                )?;
                canvas.draw(&mesh, DrawParam::default());
            }
        }
        Ok(())
    }
}
