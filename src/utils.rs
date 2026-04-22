pub fn grid_to_pixel(pos: (i32, i32)) -> (f32, f32) {
    (pos.0 as f32 * 20.0, pos.1 as f32 * 20.0)
}
