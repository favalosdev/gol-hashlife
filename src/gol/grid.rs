pub struct Grid {
    pub rows: usize, // Number of rows
    pub cols: usize, // Number of columns
    pub cells: Vec<Vec<bool>>
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let cells: Vec<Vec<bool>> = vec![vec![true; cols]; rows];
        Self { rows, cols, cells }
    }

    pub fn grow(&mut self, dx: usize, dy: usize) {
        for i in 0..(self.rows) {
            for _ in 0..dx {
                self.cells[i].insert(0, false);
                self.cells[i].push(false);
            }
        }

        for _ in 0..dy {
            let rear = vec![false; self.cols + 2*dx];
            let front = vec![false; self.cols + 2*dx];
            self.cells.insert(0, rear);
            self.cells.push(front);
        }

        self.rows += 2*dy;
        self.cols += 2*dx;
    }

    pub fn shrink(&mut self, dx: usize, dy: usize) {
        for i in 0..(self.rows) {
            for _ in 0..dx {
                self.cells[i].remove(0);
                self.cells[i].pop();
            }
        }

        for _ in 0..dy {
            self.cells.remove(0);
            self.cells.pop();
        }

        self.rows -= 2*dy;
        self.cols -= 2*dx;
    }
}
