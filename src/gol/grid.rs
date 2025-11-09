use std::collections::HashMap;

pub struct Grid<const R: usize, const C: usize> {
    cells: HashMap<(usize, usize), bool> // IMPORTANT: this uses (x,y) format
}

impl<const R: usize, const C: usize> Grid<R, C> {
    pub fn new() -> Self {
        let mut init: HashMap<(usize, usize), bool> = HashMap::new();

        init.insert((18,15), true);
        init.insert((19,15), true);
        init.insert((20,15), true);
        init.insert((18,14), true);
        init.insert((19,13), true);

        init.insert((9,25), true);
        init.insert((10,25), true);
        init.insert((10,26), true);
        init.insert((9,27), true);
        init.insert((8,26), true);

        init.insert((11,10), true);
        init.insert((12, 10), true);
        init.insert((12, 11), true);
        init.insert((12, 12), true);
        init.insert((13, 11), true);

        Self {
            cells: init
        }
    }

    pub fn evolve(&mut self) {
        let mut copy = self.cells.clone();

        for y in 0..R {
            for x in 0..C {
                let will_be_alive = self.transition(x, y);

                if will_be_alive {
                    if !self.is_alive(x, y) {
                        copy.insert((x,y), true);
                    } 
                } else {
                    if self.is_alive(x, y) {
                        copy.remove(&(x,y));
                    }
                }
            }
        }
        self.cells = copy;
    }

    fn count_alive_neighbors(&self, x: isize, y: isize) -> usize {
        let offsets = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];

        // We are modelling the grid as a toroid
        offsets.iter().map(|&(dx, dy)| {
            let grid_x = (x + dx).rem_euclid(C as isize) as usize;
            let grid_y = (y + dy).rem_euclid(R as isize) as usize;
            (self.is_alive(grid_x, grid_y) as u8) as usize
        })
        .sum()
    }

    pub fn transition(&self, x: usize, y: usize) -> bool {
        let a = self.count_alive_neighbors(x as isize, y as isize);

        if self.is_alive(x, y) {
            if a < 2 || a > 3 {
                return false;
            }
        } 

        if a == 3 {
            return true;
        }

        return self.is_alive(x, y);
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        self.cells.get(&(x,y)).is_some()
    }
}
