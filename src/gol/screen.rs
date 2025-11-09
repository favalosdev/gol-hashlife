pub struct ScreenState {
    scale: i32,
    offset_x: i32,
    offset_y: i32 
}

impl ScreenState {
    pub fn new(scale: i32, offset_x: i32, offset_y: i32) -> Self {
        Self {
            scale,
            offset_x,
            offset_y
        }
    }

    pub fn world_to_screen(&self, x_w: usize, y_w: usize, square_size: u32) -> (i32, i32) {
        let x_s: i32 = (((x_w as u32) * square_size) as i32 * self.scale) + self.offset_x;
        let y_s: i32 = (((y_w as u32) * square_size) as i32 * self.scale) + self.offset_y;
        (x_s, y_s)
    }
}
