/*
The universe of the Game of Life is an infinite, two-dimensional orthogonal grid of square
cells, each of which is in one of two possible states, alive or dead, (or populated and unpopulated,
respectively). Every cell interacts with its eight neighbours, which are the cells that are
horizontally, vertically, or diagonally adjacent. At each step in time, the following transitions
occur:

Any live cell with fewer than two live neighbors dies, as if by under population.
Any live cell with two or three live neighbors lives on to the next generation.
Any live cell with more than three live neighbors dies, as if by overpopulation.
Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

The initial pattern constitutes the seed of the system. The first generation is created by applying
the above rules simultaneously to every cell in the seed; births and deaths occur simultaneously,
and the discrete moment at which this happens is sometimes called a tick. Each generation is a pure
function of the preceding one. The rules continue to be applied repeatedly to create further
generations.
*/

use std::{thread, time};

const KILLED_ICON: char = '💔';
const BORN_ICON: char = '💚';
const SURVIVED_ICON: char = '💙';
const DEAD_ICON: char = 'b';
const WIDTH: usize = 55;
const HEIGHT: usize = 55;


#[derive(Copy, Clone)]
pub struct Cell {
    pub icon: char
}

impl Cell {

    pub fn new(icon: char) -> Self {
        Self {
            icon,
        }
    }

    pub fn update(&mut self, icon: char) {
        self.icon = icon;
    }

    pub fn is_alive(&self) -> bool {
        self.icon == BORN_ICON || self.icon == SURVIVED_ICON
    }

}

pub struct Cells {
   state: [[Cell; WIDTH]; HEIGHT]
}

impl Cells {

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            state: [[Cell::new(DEAD_ICON); WIDTH]; HEIGHT]
        }
    }

    pub fn seed(&mut self) {
        /*
        for i in 0..HEIGHT {
            self.state[i][0].update(BORN_ICON);
            self.state[i][WIDTH-1].update(BORN_ICON);
        }
        for i in 0..WIDTH {
            self.state[0][i].update(BORN_ICON);
            self.state[HEIGHT-1][i].update(BORN_ICON);
            //self.state[HEIGHT-2][i].update(KILLED_ICON);
        }
        */

        // THIS ONE TOO
        self.state[0][0].update(BORN_ICON);
        self.state[0][WIDTH-1].update(BORN_ICON);
        self.state[HEIGHT-1][0].update(BORN_ICON);
        self.state[HEIGHT-1][WIDTH-1].update(BORN_ICON);
        self.state[10][10].update(BORN_ICON);
        self.state[11][10].update(BORN_ICON);
        self.state[12][10].update(BORN_ICON);
        self.state[20][20].update(BORN_ICON);
        self.state[20][21].update(BORN_ICON);
        self.state[20][22].update(BORN_ICON);
        self.state[17][22].update(BORN_ICON);
        self.state[18][23].update(BORN_ICON);
        self.state[19][24].update(BORN_ICON);
        self.state[20][25].update(BORN_ICON);
        self.state[17][5].update(BORN_ICON);
        self.state[18][5].update(BORN_ICON);
        self.state[19][5].update(BORN_ICON);
        self.state[20][5].update(BORN_ICON);
        self.state[17][7].update(BORN_ICON);
        self.state[18][8].update(BORN_ICON);
        self.state[19][9].update(BORN_ICON);
        self.state[20][9].update(BORN_ICON);
        self.state[20][10].update(BORN_ICON);
        self.state[20][11].update(BORN_ICON);
        self.state[21][11].update(BORN_ICON);
        self.state[22][11].update(BORN_ICON);
        self.state[23][11].update(BORN_ICON);
        self.state[30][11].update(BORN_ICON);
        self.state[30][12].update(BORN_ICON);
        self.state[31][12].update(BORN_ICON);
        self.state[31][13].update(BORN_ICON);
        self.state[32][14].update(BORN_ICON);
        self.state[33][15].update(BORN_ICON);
        self.state[34][16].update(BORN_ICON);
        self.state[35][17].update(BORN_ICON);

        self.state[HEIGHT-1][WIDTH-1].update(BORN_ICON);
        self.state[HEIGHT-1][WIDTH-1].update(BORN_ICON);
        self.state[HEIGHT-1][WIDTH-1].update(BORN_ICON);

        for i in HEIGHT-12..HEIGHT-1 {
            for j in WIDTH-6..WIDTH-1 {
                self.state[i][j].update(BORN_ICON);
            }
        }

        for i in 0..8 {
            for j in 0..12 {
                self.state[i][j].update(BORN_ICON);
            }
        }

        for i in 10..25 {
            for j in 30..40 {
                self.state[i][j].update(BORN_ICON);
            }
        }
    }

    pub fn print(&self) {
        for row in self.state.iter() {
            for col in row.iter() {
                match col.icon {
                    'b' => print!("   "),
                    _ => print!("{} ", col.icon),
                }
            }
            println!();
        }
    }

}

pub struct Generation {
    pub n: u64,
    pub tick_rate: u64,
    pub born: u64,
    pub killed: u64,
    pub survived: u64,
}

pub struct Game {
    pub cells: Cells,
    pub width: usize,
    pub height: usize,
    pub generation: Generation,
}

impl Game {

    pub fn new(width: usize, height: usize, tick_rate: u64) -> Self {
        let mut cells = Cells::new(width, height);
        cells.seed();
        Self {
            cells,
            height,
            width,
            generation: Generation {
                n: 0,
                tick_rate,
                born: 0,
                killed: 0,
                survived: 0,
            }
        }
    }

    pub fn print(&self) {
        &self.cells.print();
        println!("\nTick Rate: {}ms // Generation {: <5} // Killed {: <3} // Survived {: <3} // Born {: <3}",
                 &self.generation.tick_rate,
                 &self.generation.n, &self.generation.killed,
                 &self.generation.survived, &self.generation.born);
    }

    pub fn tick(&mut self) {
        /*
        Any live cell:
          with fewer than two live neighbors dies (under population)
          with two or three live neighbors lives
          with more than three live neighbors dies (over population)
        Any dead cell:
          with exactly three live neighbors becomes a live cell (reproduction)
        */

        self.generation.n += 1;
        self.generation.born = 0;
        self.generation.killed = 0;
        self.generation.survived = 0;

        let mut f_cells = Cells::new(self.width, self.height);

        for (i, row) in self.cells.state.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                //match *col {
                //    ALIVE => self.handle_live(*col),
                //    DEAD => self.handle_dead(*col),
                //    _ => println!("Fuck."),
                //}
                //*col = 'l';

                let l = self.cells.state.len() - 1;
                let mut n = 0;

                // let mut v = [u8, 3];
                // v.push(-1);
                // v.push(0);
                // v.push(1);
                
                // let mut k = Vec::new();
                // k.push(-1);
                // k.push(0);
                // k.push(1);

                // for i in v.into_iter() {
                //     if i < 0 || i > HEIGHT {
                //         continue
                //     }
                //     for j in k.into_iter() {
                //         if j < 0 || j > WIDTH {
                //             if self.cells.state[i][j].is_alive() {
                //                 n += 1
                //             }
                //         }
                //     }
                // }

                if i > 0 && j > 0 && self.cells.state[i-1][j-1].is_alive() {
                    n += 1
                }
                if j > 0 && self.cells.state[i][j-1].is_alive() {
                    n += 1
                }
                if i < l && j > 0 && self.cells.state[i+1][j-1].is_alive() {
                    n += 1
                }
                if n < 4 && i > 0 && self.cells.state[i-1][j].is_alive() {
                    n += 1
                }
                if n < 4 && i < l && self.cells.state[i+1][j].is_alive() {
                    n += 1
                }
                if n < 4 && i > 0 && j < l && self.cells.state[i-1][j+1].is_alive() {
                    n += 1
                }
                if n < 4 && j < l && self.cells.state[i][j+1].is_alive() {
                    n += 1
                }
                if n < 4 && i < l && j < l && self.cells.state[i+1][j+1].is_alive() {
                    n += 1
                }

                if col.is_alive() {
                    if n < 2 || n > 3 {
                        f_cells.state[i][j].update(KILLED_ICON);
                        self.generation.killed += 1;
                    } else {
                         f_cells.state[i][j].update(SURVIVED_ICON);
                        self.generation.survived += 1;
                    }
                } else {
                    if n == 3 {
                        f_cells.state[i][j].update(BORN_ICON);
                        self.generation.born += 1
                    }
                }
            }
        }

        self.cells = f_cells;
        thread::sleep(time::Duration::from_millis(self.generation.tick_rate));
    }
}
