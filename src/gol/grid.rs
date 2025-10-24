use std::collections::HashSet;

pub struct Grid<const N: usize, const M: usize> {
    cells: HashSet<(i32, i32)>, // We will opt for an (x, y) format
}

impl<const N: usize, const M: usize> Grid<N, M> {
    pub fn new() -> Self {
        let mut cells: HashSet<(i32, i32)> = HashSet::new();

        cells.insert((18, 15));
        cells.insert((19, 15));
        cells.insert((20, 15));
        cells.insert((18, 14));
        cells.insert((19, 13));

        cells.insert((9, 25));
        cells.insert((10, 25));
        cells.insert((10, 26));
        cells.insert((9, 27));
        cells.insert((8, 26));

        cells.insert((11, 10));
        cells.insert((12, 10));
        cells.insert((12, 11));
        cells.insert((12, 12));
        cells.insert((13, 11));

        Self { cells }
    }

    pub fn evolve(&mut self) {
        for coords_ref in self.cells.iter() {
            cells.insert
        }
    }

    fn count_alive_neighbors(&self, x: isize, y: isize) -> usize {
        let m_p = M as isize;
        let n_p = N as isize;

        let offsets = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];

        // We are modelling the grid as a toroid
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
