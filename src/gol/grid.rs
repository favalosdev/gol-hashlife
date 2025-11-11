use std::collections::{HashSet, LinkedList};

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

    pub fn evolve(&mut self) {
        let mut to_traverse: LinkedList<(isize, isize)> = LinkedList::new();

        for (x,y) in self.cells.iter() {
            let (x, y) = (*x, *y);

            to_traverse.push_back((x, y));
            let mut neighbors = self.get_neighbor_coords(x, y);
            to_traverse.append(&mut neighbors)
        }

        let mut copy: HashSet<(isize, isize)> = HashSet::new();

        for (x, y) in to_traverse.iter() {
            let (x, y) = (*x, *y);
            let will_be_alive = self.transition(x, y);

            if will_be_alive {
                copy.insert((x, y));
            }
        }

        self.cells = copy;
    }

    pub fn get_neighbor_coords(&self, x: isize, y: isize) -> LinkedList<(isize, isize)> {
        let offsets = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];

        let coords: LinkedList<(isize,isize)> = offsets.iter().map(|&(dx, dy)| {
            let mut x_f= x + dx;
            let mut y_f = y + dy;

            if x_f.abs() == self.range {
                x_f = x.abs() * (-1) * dx.signum();
            }

            if y_f.abs() == self.range {
                y_f = y.abs() * (-1) * dy.signum();
            }

            (x_f, y_f)
        }).collect();

        return coords;
    }

    fn count_alive_neighbors(&self, x: isize, y: isize) -> usize {
        self.get_neighbor_coords(x, y).iter().map(|&(x,y)| self.is_alive(x, y) as usize).sum()
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
