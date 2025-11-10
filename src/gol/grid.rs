use std::collections::HashSet;

pub struct Grid {
    pub cells: HashSet<(isize, isize)> // IMPORTANT: this uses (x,y) format
}

impl Grid {
    pub fn new() -> Self {
        let mut cells: HashSet<(isize, isize)> = HashSet::new();

        cells.insert((0,2));
        cells.insert((1,2));
        cells.insert((2,2));
        cells.insert((0,1));
        cells.insert((1,0));

        cells.insert((-9,12));
        cells.insert((-8,12));
        cells.insert((-8,13));
        cells.insert((-9,14));
        cells.insert((-10,13));

        cells.insert((-7,-3));
        cells.insert((-6,-3));
        cells.insert((-6,-2));
        cells.insert((-6,-1));
        cells.insert((-5,-2));

        Self {
            cells
        }
    }

    pub fn evolve(&mut self) {
        let mut copy = self.cells.clone();

        for (x,y) in self.cells.iter() {
            let will_be_alive = self.transition(*x,*y);

            if will_be_alive {
                if !self.is_alive(*x,*y) {
                    copy.insert((*x,*y));
                } 
            } else {
                if self.is_alive(*x,*y) {
                    copy.remove(&(*x,*y));
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

        offsets.iter().map(|&(dx, dy)| {
            self.is_alive(x + dx, y + dy) as usize
        })
        .sum()
    }

    pub fn transition(&self, x: isize, y: isize) -> bool {
        let a = self.count_alive_neighbors(x, y);

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

    pub fn is_alive(&self, x: isize, y: isize) -> bool {
        self.cells.get(&(x,y)).is_some()
    }
}
