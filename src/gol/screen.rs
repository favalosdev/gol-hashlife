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

    pub fn world_to_screen(&self, x_w: usize, y_w: usize, square_size: usize) -> (i32, i32) {
        let x_s = (x_w * square_size) as i32;
        let y_s = (y_w * square_size) as i32;
        (x_s, y_s)
    }
}
