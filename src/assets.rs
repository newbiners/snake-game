use ggez::graphics::Image;
use ggez::{Context, GameResult};
pub struct Assets {
    pub apple: Image,
    pub head_up: Image,
    pub head_down: Image,
    pub head_left: Image,
    pub head_right: Image,
    pub body_horz: Image,
    pub body_vert: Image,
    pub body_tl: Image,
    pub body_tr: Image,
    pub body_bl: Image,
    pub body_br: Image,
    pub tail_up: Image,
    pub tail_down: Image,
    pub tail_left: Image,
    pub tail_right: Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            apple: Image::from_path(ctx, "/apple.png")?,
            head_up: Image::from_path(ctx, "/head_up.png")?,
            head_down: Image::from_path(ctx, "/head_down.png")?,
            head_left: Image::from_path(ctx, "/head_left.png")?,
            head_right: Image::from_path(ctx, "/head_right.png")?,
            body_horz: Image::from_path(ctx, "/body_horizontal.png")?,
            body_vert: Image::from_path(ctx, "/body_vertical.png")?,
            body_tl: Image::from_path(ctx, "/body_topleft.png")?,
            body_tr: Image::from_path(ctx, "/body_topright.png")?,
            body_bl: Image::from_path(ctx, "/body_bottomleft.png")?,
            body_br: Image::from_path(ctx, "/body_bottomright.png")?,
            tail_up: Image::from_path(ctx, "/tail_up.png")?,
            tail_down: Image::from_path(ctx, "/tail_down.png")?,
            tail_left: Image::from_path(ctx, "/tail_left.png")?,
            tail_right: Image::from_path(ctx, "/tail_right.png")?,
        })
    }
}
