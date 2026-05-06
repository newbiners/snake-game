use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color, DrawParam, Mesh, PxScale, Text, TextFragment};
use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::food::Food;
use crate::snake::Snake;
use crate::wall::Wall;

pub struct GameState {
    snake: Snake,
    food: Food,
    assets: Assets,
    wall: Wall,
    game_over: bool,
    score: u32,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            assets: Assets::new(ctx)?,
            snake: Snake::new(),
            food: Food::new(),
            wall: Wall::new(),
            game_over: false,
            score: 0,
        })
    }
    fn restart(&mut self) {
        self.snake = Snake::new();
        self.food = Food::new();
        self.wall = Wall::new();
        self.game_over = false;
        self.score = 0;
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Jangan update game kalau game over
        if self.game_over {
            return Ok(());
        }

        let dt = ctx.time.delta().as_secs_f32();
        self.snake.update(dt);

        // Teleportasi: wrap kepala ular ke sisi berlawanan jika keluar batas
        crate::teleportation::wrap_position(&mut self.snake.body[0]);

        if self.snake.head_position() == self.food.position {
            self.snake.grow();
            self.food.respawn();
            self.wall.respawn(); // Pindahkan posisi tembok setiap kali makan
            self.score += 1;
        }

        // Game over: ular menabrak dirinya sendiri atau tembok
        if self.snake.collides_with_self() || self.wall.collides_with(self.snake.head_position()) {
            self.game_over = true;
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        use ggez::input::keyboard::KeyCode;

        if let Some(keycode) = input.keycode {
            // Kalau game over, tekan Enter atau Space untuk restart
            if self.game_over {
                if keycode == KeyCode::Return || keycode == KeyCode::Space {
                    self.restart();
                }
                return Ok(());
            }

            let new_dir = match keycode {
                KeyCode::Up => Some((0, -1)),
                KeyCode::Down => Some((0, 1)),
                KeyCode::Left => Some((-1, 0)),
                KeyCode::Right => Some((1, 0)),
                _ => None,
            };

            if let Some(dir) = new_dir {
                // Prevent reversing into yourself
                if (
                    dir.0 + self.snake.direction.0,
                    dir.1 + self.snake.direction.1,
                ) != (0, 0)
                {
                    self.snake.direction = dir;
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.wall.draw(&mut canvas, ctx)?;
        self.snake.draw(&mut canvas, &self.assets)?;
        self.food.draw(&mut canvas, &self.assets)?;

        // Tampilkan skor di pojok kiri atas
        let score_text = Text::new(TextFragment {
            text: format!("Score: {}", self.score),
            color: Some(Color::WHITE),
            scale: Some(PxScale::from(24.0)),
            ..Default::default()
        });
        canvas.draw(&score_text, DrawParam::default().dest([10.0, 10.0]));

        // Tampilkan popup Game Over
        if self.game_over {
            let (screen_w, screen_h) = ctx.gfx.drawable_size();

            // Semi-transparent dark overlay
            let overlay = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, screen_w, screen_h),
                Color::new(0.0, 0.0, 0.0, 0.75),
            )?;
            canvas.draw(&overlay, DrawParam::default());

            // "GAME OVER" title
            let game_over_text = Text::new(TextFragment {
                text: "GAME OVER".to_string(),
                color: Some(Color::new(1.0, 0.3, 0.3, 1.0)), // Merah
                scale: Some(PxScale::from(48.0)),
                ..Default::default()
            });
            let go_width = game_over_text.measure(ctx)?.x;
            canvas.draw(
                &game_over_text,
                DrawParam::default().dest([(screen_w - go_width) / 2.0, screen_h / 2.0 - 60.0]),
            );

            // Skor akhir
            let final_score_text = Text::new(TextFragment {
                text: format!("Score: {}", self.score),
                color: Some(Color::YELLOW),
                scale: Some(PxScale::from(32.0)),
                ..Default::default()
            });
            let fs_width = final_score_text.measure(ctx)?.x;
            canvas.draw(
                &final_score_text,
                DrawParam::default().dest([(screen_w - fs_width) / 2.0, screen_h / 2.0]),
            );

            // Instruksi restart
            let restart_text = Text::new(TextFragment {
                text: "Press ENTER or SPACE to restart".to_string(),
                color: Some(Color::new(0.7, 0.7, 0.7, 1.0)), // Abu-abu terang
                scale: Some(PxScale::from(20.0)),
                ..Default::default()
            });
            let rt_width = restart_text.measure(ctx)?.x;
            canvas.draw(
                &restart_text,
                DrawParam::default().dest([(screen_w - rt_width) / 2.0, screen_h / 2.0 + 50.0]),
            );
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}
