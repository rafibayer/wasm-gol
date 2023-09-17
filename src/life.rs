#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Cell {
    Dead = 0,

    #[default]
    Alive = 1,
}

// implement unary ! for Cell type
impl std::ops::Not for Cell {
    type Output = Cell;

    fn not(self) -> Self::Output {
        match self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

// world is a contiguous Vec of cells,
// we rely on width and height to determine
// which row and column a cell belongs to
pub struct World {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new() -> Self {
        let width = 100;
        let height = 100;

        // starting state, every 2nd and 7th cell is alive
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Self {
            cells,
            width,
            height,
        }
    }

    #[inline]
    pub fn tick(&mut self) {
        let mut next_cells = vec![Cell::Dead; self.width * self.height];

        // for each cell, count live neighbors, update next_cells state
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_cell_index(row, col);
                let neighbors = self.get_live_neighbor_count(row, col);
                let state = self.cells[index];

                let next = match (state, neighbors) {
                    (Cell::Alive, 2) => Cell::Alive,
                    (_, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };

                next_cells[index] = next;
            }
        }

        self.cells = next_cells;
    }

    #[inline]
    fn get_live_neighbor_count(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        // we use bounds-1 rather than -1 to avoid
        // unsigned int underflow and height/width of 0.
        // modulo operator takes care of wrapping.
        for d_row in [self.height - 1, 0, 1] {
            for d_col in [self.width - 1, 0, 1] {
                if d_row == 0 && d_col == 0 {
                    continue;
                }

                let new_row = (row + d_row) % self.height;
                let new_col = (col + d_col) % self.width;
                let index = self.get_cell_index(new_row, new_col);
                count += self.cells[index] as u8;
            }
        }

        count
    }

    #[inline]
    pub fn get_cell_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const X: Cell = Cell::Dead;
    const O: Cell = Cell::Alive;

    #[test]
    fn test_get_cell_index() {
        let world = World {
            cells: vec![
                X, X, X, X, // 0-3
                X, X, X, X, // 4-7
                X, X, X, X, // 8-11
            ],
            width: 4,
            height: 3,
        };

        assert_eq!(11, world.get_cell_index(2, 3));
        assert_eq!(0, world.get_cell_index(0, 0));
        assert_eq!(8, world.get_cell_index(2, 0));
    }

    #[test]
    fn test_get_live_neighbor_count() {
        let world = World {
            cells: vec![
                O, O, X, X, // 0-3
                X, O, X, X, // 4-7
                X, X, X, X, // 8-11
            ],
            width: 4,
            height: 3,
        };

        assert_eq!(2, world.get_live_neighbor_count(1, 1));
        assert_eq!(1, world.get_live_neighbor_count(2, 3));
    }

    #[test]
    fn test_tick() {
        // spinner test
        let mut world = World {
            cells: vec![
                X, X, X, X, X, //
                X, X, O, X, X, //
                X, X, O, X, X, //
                X, X, O, X, X, //
                X, X, X, X, X, //
            ],
            width: 5,
            height: 5,
        };

        let original_cells = world.cells.clone();

        world.tick();

        assert_eq!(
            world.cells,
            vec![
                X, X, X, X, X, //
                X, X, X, X, X, //
                X, O, O, O, X, //
                X, X, X, X, X, //
                X, X, X, X, X, //
            ]
        );

        world.tick();

        assert_eq!(world.cells, original_cells);
    }
}
