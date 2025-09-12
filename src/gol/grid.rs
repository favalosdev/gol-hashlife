pub struct Grid<const N: usize, const M: usize> {
    grid: [[bool; M]; N]
}

impl<const N: usize, const M: usize> Grid<N, M> {
    pub fn new() -> Self {
        let mut init: [[bool;M]; N] = [[false;M]; N];

        // Our way of initializing the seed
        init[15][18] = true;
        init[15][19] = true;
        init[15][20] = true;
        init[14][18] = true;
        init[13][19] = true;

        init[25][9] = true;
        init[25][10] = true;
        init[26][10] = true;
        init[27][9] = true;
        init[26][8] = true;

        Self {
            grid: init
        }
    }

    pub fn evolve(&mut self) {
        let mut copy: [[bool;M]; N] = [[false;M]; N];

        for y in 0..N {
            for x in 0..M {
                let value = self.transition(x, y);
                copy[y][x] = value;
            }
        }
        self.grid = copy;
    }

    pub fn transition(&self, x: usize, y: usize) -> bool {
        let x_p: i32 = x as i32;
        let y_p: i32 = y as i32;
        let positions: [(i32, i32); 8] = [(x_p-1,y_p),(x_p-1,y_p+1),(x_p,y_p+1),(x_p+1,y_p+1),(x_p+1,y_p),(x_p+1,y_p-1),(x_p,y_p-1),(x_p-1,y_p-1)];
        let neighbors = c![self.grid[p.1 as usize][p.0 as usize], for p in positions, if 0 <= p.0 && p.0 < (M as i32) && 0 <= p.1 && p.1 < (N as i32)];
        let alive: usize = c![n, for n in neighbors, if n == true].len();

        if self.grid[y][x] {
            if alive < 2 || alive > 3 {
                return false;
            }
        } 

        if alive == 3 {
            return true;
        }

        return self.grid[y][x];
    }

    pub fn retrieve(&self, x: usize, y: usize) -> bool {
        self.grid[y][x]
    }
}
