use std::collections::HashSet;

pub struct Grid {
    pub cells: HashSet<(isize, isize)>, // IMPORTANT: this uses (x,y) format
    range_x: isize,
    range_y: isize
}

impl Grid {
    pub fn new() -> Self {
        let mut cells: HashSet<(isize, isize)> = HashSet::new();

        cells.insert((0,2));
        cells.insert((1,2));
        cells.insert((2,2));
        cells.insert((2,3));
        cells.insert((1,4));

        cells.insert((-9,12));
        cells.insert((-8,12));
        cells.insert((-8,13));
        cells.insert((-9,14));
        cells.insert((-10,13));

        cells.insert((-6,-3));
        cells.insert((-6,-2));
        cells.insert((-6,-1));
        cells.insert((-7,-1));
        cells.insert((-8,-2));

        Self {
            cells,
            range_x: 20,
            range_y: 20 
        }
    }

    pub fn evolve(&mut self) {
        let mut copy = self.cells.clone();

        for x in -self.range_x..self.range_x {
            for y in -self.range_y..self.range_y {
                let will_be_alive = self.transition(x,y);

                if will_be_alive {
                    if !self.is_alive(x,y) {
                        copy.insert((x,y));
                    } 
                } else {
                    if self.is_alive(x,y) {
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

        offsets.iter().map(|&(dx, dy)| {
            let mut x_f= x + dx;
            let mut y_f = y + dy;

            if x_f.abs() == self.range_x {
                x_f = x * (-dx.signum());
            }

            if y_f.abs() == self.range_y {
                y_f = y * (-dy.signum());
            }

            self.is_alive(x_f, y_f) as usize
        })
        .sum()
    }

    pub fn transition(&self, x: isize, y: isize) -> bool {
        let n = self.count_alive_neighbors(x, y);

        if self.is_alive(x, y) {
            if n < 2 || n > 3 {
                return false;
            }
        } 

        if n == 3 {
            return true;
        }

        return self.is_alive(x, y);
    }

    pub fn is_alive(&self, x: isize, y: isize) -> bool {
        self.cells.get(&(x,y)).is_some()
    }
}
