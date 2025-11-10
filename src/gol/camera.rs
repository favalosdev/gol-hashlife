pub struct Camera {
    pub zoom: i32,
    pub x: i32,
    pub y: i32
}

impl Camera {
    pub fn new(zoom: i32, x: i32, y: i32) -> Self {
        Self { zoom, x, y }
    }

    pub fn from_world_coords(&self, x_w: usize, y_w: usize) -> (i32, i32) {
        let x_s = ((x_w as i32) - self.x) * self.zoom;
        let y_s = ((y_w as i32) - self.y) * self.zoom;
        (x_s, y_s)
    }
}
