pub struct Grid<const N: usize, const M: usize> {
    grid: [[bool; M]; N]
}

impl<const N: usize, const M: usize> Grid<N, M> {
    pub fn new() -> Self {
        Self {
            grid: [[false; M]; N]
        }
    }

    pub fn evolve(&mut self) {
        for y in 0..N {
            for x in 0..M {
                let value = self.transition(x, y);
                self.grid[y][x] = value;
            }
        }
    }

    pub fn transition(&self, x: usize, y: usize) -> bool {
        !self.grid[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.grid[y][x] = value;
    }

    pub fn retrieve(&self, x: usize, y: usize) -> bool {
        self.grid[y][x]
    }
}
