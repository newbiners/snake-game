use ggez::{Context, GameResult};

pub struct Snake {
    pub body: Vec<(i32, i32)>,
    pub direction: (i32, i32),
    pub move_delay: f32, // Target waktu (misal 0.15 detik)
    pub timer: f32,      // Penghitung waktu yang berjalan
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![(10, 10), (10, 11), (10, 12)],
            direction: (0, -1),
            move_delay: 0.15, // Semakin besar angka ini, semakin lambat ularnya (0.15 = 150ms)
            timer: 0.0,
        }
    }

    // pub fn update(&mut self) {
    //     let mut new_head = self.body[0];

    //     new_head.0 += self.direction.0;
    //     new_head.1 += self.direction.1;

    //     self.body.insert(0, new_head);
    //     self.body.pop();
    // }
    pub fn update(&mut self, dt: f32) {
        self.timer += dt;

        // Ular hanya akan bergerak jika timer sudah melewati move_delay
        if self.timer >= self.move_delay {
            let mut new_head = self.body[0];
            new_head.0 += self.direction.0;
            new_head.1 += self.direction.1;

            self.body.insert(0, new_head);
            self.body.pop();

            // Reset timer kembali ke nol
            self.timer = 0.0;
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        self.body[0]
    }
    pub fn collides_with_self(&self) -> bool {
        let head = self.body.first().unwrap(); // Ambil posisi kepala

        // Cek semua bagian tubuh KECUALI kepala (index 0)
        // Kita gunakan .iter().skip(1) untuk melewati kepala
        for segment in self.body.iter().skip(1) {
            if head == segment {
                return true; // Terjadi tabrakan!
            }
        }
        false // Tidak ada tabrakan
    }

    pub fn grow(&mut self) {
        let tail = *self.body.last().unwrap();
        self.body.push(tail);
    }

    pub fn draw(&self, canvas: &mut ggez::graphics::Canvas, ctx: &ggez::Context) -> GameResult {
        use crate::utils::grid_to_pixel;
        use ggez::graphics::{self, Color, DrawParam, Mesh};

        for &segment in &self.body {
            let pos = grid_to_pixel(segment);
            let mesh = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(pos.0, pos.1, 20.0, 20.0),
                Color::GREEN,
            )?;
            canvas.draw(&mesh, DrawParam::default());
        }
        Ok(())
    }
}
