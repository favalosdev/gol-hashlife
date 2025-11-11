use std::collections::HashSet;

#[derive(Clone)]
pub struct Grid {
    pub cells: HashSet<(isize, isize)>, // Important: this uses (x,y) format
    range: isize
}

impl Grid {
    pub fn new(range: isize) -> Self {
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
            range
        }
    }

    pub fn is_alive(&self, x: isize, y: isize) -> bool {
        self.cells.get(&(x,y)).is_some()
    }

    pub fn enliven(&mut self, x: isize, y: isize) {
        self.cells.insert((x,y));
    }

    pub fn kill(&mut self, x: isize, y: isize) {
        self.cells.remove(&(x,y));
    }

    pub fn evolve(&mut self) {
        let mut copy = self.clone();

        for x in (-self.range)+1..self.range {
            for y in -self.range+1..self.range {
                let will_be_alive = self.transition(x,y);

                if will_be_alive {
                    if !self.is_alive(x,y) {
                        copy.enliven(x,y);
                    } 
                } else {
                    if self.is_alive(x,y) {
                        copy.kill(x,y);
                    }
                }
            }
        }

        self.cells = copy.cells;
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

            if x_f.abs() == self.range {
                x_f = x.abs() * (-1) * dx.signum();
            }

            if y_f.abs() == self.range {
                y_f = y.abs() * (-1) * dy.signum();
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
}
