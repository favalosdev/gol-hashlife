use std::collections::{HashSet, LinkedList};
use ca_formats::rle::Rle;

const RANGE: usize = 2000;

pub struct Grid {
    pub cells: HashSet<(isize, isize)> // Important: this uses (x,y) format
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: HashSet::new()
        }
    }

    pub fn load_pattern<T : ca_formats::Input>(&mut self, pattern: Rle<T>) {
        let width = pattern.header_data().unwrap().x;
        let height = pattern.header_data().unwrap().y;
        self.cells = pattern
            .map(|cell| cell.unwrap())
            .filter(|data | data.state == 1)
            .map(|data| ((data.position.0 - (width as i64) / 2) as isize, (data.position.1 - (height as i64) / 2) as isize))
            .collect::<HashSet<_>>();
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

            if x_f.abs() as usize == RANGE {
                x_f = x.abs() * (-1) * dx.signum();
            }

            if y_f.abs() as usize == RANGE {
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
