/*
The universe of the GoL of Life is an infinite, two-dimensional orthogonal grid of square
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


pub struct Generation {
    state: [[char; WIDTH]; HEIGHT],
    n: u64,
    born: u64,
    killed: u64,
    survived: u64,
}

impl Generation {

    pub fn new() -> Self {
        Self {
            state: [[DEAD_ICON; WIDTH]; HEIGHT],
            n: 0,
            born: 0,
            killed: 0,
            survived: 0,
        }
    }

    pub fn seed(&mut self) {
        /*
        for i in 0..HEIGHT {
            self.state[i][0] = BORN_ICON;
            self.state[i][WIDTH-1] = BORN_ICON;
        }
        for i in 0..WIDTH {
            self.state[0][i] = BORN_ICON;
            self.state[HEIGHT-1][i] = BORN_ICON;
            //self.state[HEIGHT-2][i] = KILLED_ICON;
        }
        */

        // THIS ONE TOO
        self.state[0][0] = BORN_ICON;
        self.state[0][WIDTH-1] = BORN_ICON;
        self.state[HEIGHT-1][0] = BORN_ICON;
        self.state[HEIGHT-1][WIDTH-1] = BORN_ICON;
        self.state[10][10] = BORN_ICON;
        self.state[11][10] = BORN_ICON;
        self.state[12][10] = BORN_ICON;
        self.state[20][20] = BORN_ICON;
        self.state[20][21] = BORN_ICON;
        self.state[20][22] = BORN_ICON;
        self.state[17][22] = BORN_ICON;
        self.state[18][23] = BORN_ICON;
        self.state[19][24] = BORN_ICON;
        self.state[20][25] = BORN_ICON;
        self.state[17][5] = BORN_ICON;
        self.state[18][5] = BORN_ICON;
        self.state[19][5] = BORN_ICON;
        self.state[20][5] = BORN_ICON;
        self.state[17][7] = BORN_ICON;
        self.state[18][8] = BORN_ICON;
        self.state[19][9] = BORN_ICON;
        self.state[20][9] = BORN_ICON;
        self.state[20][10] = BORN_ICON;
        self.state[20][11] = BORN_ICON;
        self.state[21][11] = BORN_ICON;
        self.state[22][11] = BORN_ICON;
        self.state[23][11] = BORN_ICON;
        self.state[30][11] = BORN_ICON;
        self.state[30][12] = BORN_ICON;
        self.state[31][12] = BORN_ICON;
        self.state[31][13] = BORN_ICON;
        self.state[32][14] = BORN_ICON;
        self.state[33][15] = BORN_ICON;
        self.state[34][16] = BORN_ICON;
        self.state[35][17] = BORN_ICON;

        self.state[HEIGHT-1][WIDTH-1] = BORN_ICON;
        self.state[HEIGHT-1][WIDTH-1] = BORN_ICON;
        self.state[HEIGHT-1][WIDTH-1] = BORN_ICON;

        for i in HEIGHT-12..HEIGHT-1 {
            for j in WIDTH-6..WIDTH-1 {
                self.state[i][j] = BORN_ICON;
            }
        }

        for i in 0..8 {
            for j in 0..12 {
                self.state[i][j] = BORN_ICON;
            }
        }

        for i in 10..25 {
            for j in 30..40 {
                self.state[i][j] = BORN_ICON;
            }
        }
    }

    pub fn tick(&mut self) -> Generation {
        let mut f_generation = Generation::new();

        for (i, row) in self.state.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {

                let n = self.neighbor_count(i, j);

                if self.is_cell_alive(i, j) {
                    if n < 2 || n > 3 {
                        f_generation.state[i][j] = KILLED_ICON;
                        f_generation.killed += 1;
                    } else {
                        f_generation.state[i][j] = SURVIVED_ICON;
                        f_generation.survived += 1;
                    }
                } else {  // is_dead, essentially :)
                    if n == 3 {
                        f_generation.state[i][j] = BORN_ICON;
                        f_generation.born += 1
                    }
                }
            }
        }

        f_generation.n = self.n + 1;

        return f_generation
    }

    pub fn is_cell_alive(&self, i: usize, j: usize) -> bool {
        match self.state[i][j] {
            BORN_ICON => true,
            SURVIVED_ICON => true,
            _ => false,
        }
    }

    pub fn neighbor_count(&self, i: usize, j: usize) -> i8 {
        let l = self.state.len() - 1;
        let mut n = 0;

        // Above left.
        if i > 0 && j > 0 && self.is_cell_alive(i-1, j-1) {
            n += 1
        }
        // Direct above.
        if i > 0 && self.is_cell_alive(i-1, j) {
            n += 1
        }
        // Above right.
        if i > 0 && j < l && self.is_cell_alive(i-1, j+1) {
            n += 1
        }
        // Direct left.
        if j > 0 && self.is_cell_alive(i, j-1) {
            n += 1
        }
        // Direct right.
        if j < l && self.is_cell_alive(i, j+1) {
            n += 1
        }
        // Below left.
        if i < l && j > 0 && self.is_cell_alive(i+1, j-1) {
            n += 1
        }
        // Direct below.
        if i < l && self.is_cell_alive(i+1, j) {
            n += 1
        }
        // Below right.
        if i < l && j < l && self.is_cell_alive(i+1, j+1) {
            n += 1
        }

        n
    }

    pub fn print(&self) {
        for row in self.state.iter() {
            for col in row.iter() {
                match col {
                    'b' => print!("   "),
                    _ => print!("{} ", col),
                }
            }
            println!();
        }

        // Generation stats.
        println!("\nGeneration {: <4} // Killed {: <3} // Survived {: <3} // Born {: <3}",
                 &self.n, &self.killed, &self.survived, &self.born);
    }

}

pub struct GoL {
    width: usize,
    height: usize,
    tick_rate: u64,
    generation: Generation,
}

impl GoL {

    pub fn new(tick_rate: u64) -> Self {
        let mut generation = Generation::new();
        generation.seed();

        Self {
            tick_rate,
            generation,
            height: HEIGHT,
            width: WIDTH,
        }
    }

    pub fn print(&self) {
        // Clears the screen.
        print!("{}[2J", 27 as char);

        // Print all Generation.
        &self.generation.print();
    }

    pub fn start(&mut self) {
        loop {
            self.generation.print();
            self.generation = self.generation.tick();
            thread::sleep(time::Duration::from_millis(self.tick_rate));
        }
    }
}
