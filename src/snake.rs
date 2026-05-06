use ggez::GameResult;

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

    pub fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        assets: &crate::assets::Assets,
    ) -> GameResult {
        use crate::utils::grid_to_pixel;
        use ggez::graphics::DrawParam;

        for (i, &segment) in self.body.iter().enumerate() {
            let pos = grid_to_pixel(segment);
            let img = if i == 0 {
                // Head
                match self.direction {
                    (0, -1) => &assets.head_up,
                    (0, 1) => &assets.head_down,
                    (-1, 0) => &assets.head_left,
                    (1, 0) => &assets.head_right,
                    _ => &assets.head_up,
                }
            } else if i == self.body.len() - 1 {
                // Tail
                let prev = self.body[i - 1];
                let dir = (segment.0 - prev.0, segment.1 - prev.1);
                match dir {
                    (0, 1) => &assets.tail_down,
                    (0, -1) => &assets.tail_up,
                    (1, 0) => &assets.tail_right,
                    (-1, 0) => &assets.tail_left,
                    _ => &assets.tail_down,
                }
            } else {
                // Body
                let prev = self.body[i - 1];
                let next = self.body[i + 1];

                let d1 = (prev.0 - segment.0, prev.1 - segment.1);
                let d2 = (next.0 - segment.0, next.1 - segment.1);

                if d1.0 == d2.0 {
                    &assets.body_vert
                } else if d1.1 == d2.1 {
                    &assets.body_horz
                } else {
                    // Corners
                    match (d1, d2) {
                        ((1, 0), (0, 1)) | ((0, 1), (1, 0)) => &assets.body_br,
                        ((-1, 0), (0, 1)) | ((0, 1), (-1, 0)) => &assets.body_bl,
                        ((1, 0), (0, -1)) | ((0, -1), (1, 0)) => &assets.body_tr,
                        ((-1, 0), (0, -1)) | ((0, -1), (-1, 0)) => &assets.body_tl,
                        _ => &assets.body_horz,
                    }
                }
            };

            canvas.draw(
                img,
                DrawParam::default().dest([pos.0, pos.1]).scale([0.5, 0.5]),
            );
        }
        Ok(())
    }
}
