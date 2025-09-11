pub struct Grid<const N: usize, const M: usize> {
    grid: [[bool; M]; N]
}

impl<const N: usize, const M: usize> Grid<N, M> {
    pub fn new() -> Self {
        let mut init: [[bool;M]; N] = [[false;M]; N];

        // Our way of initializing the seed
        init[2][1] = true;
        init[2][2] = true;
        init[3][2] = true;
        init[4][2] = true;
        init[4][3] = true;

        Self {
            grid: init
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
        let x_p: i32 = x as i32;
        let y_p: i32 = y as i32;
        let positions: [(i32, i32); 8] = [(x_p-1,y_p),(x_p-1,y_p+1),(x_p,y_p+1),(x_p+1,y_p+1),(x_p+1,y_p),(x_p+1,y_p-1),(x_p,y_p-1),(x_p-1,y_p-1)];
        let neighbors = c![self.grid[p.1 as usize][p.0 as usize], for p in positions, if 0 <= p.0 && p.0 < (M as i32) && 0 <= p.1 && p.1 < (N as i32)];
        let n_neighbors: usize = neighbors.len();

        if self.grid[y][x] {
            if n_neighbors < 2 || n_neighbors > 3 {
                return false;
            }
        } 

        if n_neighbors == 3 {
            return true;
        }

        // Otherwise return what already existed
        return self.grid[y][x];
    }


    pub fn retrieve(&self, x: usize, y: usize) -> bool {
        self.grid[y][x]
    }
}
