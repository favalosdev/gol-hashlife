use std::num::Wrapping;

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

        init[10][11] = true;
        init[10][12] = true;
        init[11][12] = true;
        init[12][12] = true;
        init[11][13] = true;

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

    fn count_alive_neighbors(&self, x: isize, y: isize) -> usize {
        let m_p = M as isize;
        let n_p = N as isize;

        let offsets = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];

        offsets.iter().map(|&(dx, dy)| {
            let grid_x = (x + dx).rem_euclid(m_p) as usize;
            let grid_y = (y + dy).rem_euclid(n_p) as usize;
            self.grid[grid_y][grid_x] as usize
        })
        .sum()
    } 

    pub fn transition(&self, x: usize, y: usize) -> bool {
        let a = self.count_alive_neighbors(x as isize, y as isize);

        if self.grid[y][x] {
            if a < 2 || a > 3 {
                return false;
            }
        } 

        if a == 3 {
            return true;
        }

        return self.grid[y][x];
    }

    pub fn retrieve(&self, x: usize, y: usize) -> bool {
        self.grid[y][x]
    }
}
