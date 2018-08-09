/*
The universe of the GoL of Life is an infinite, two-dimensional orthogonal grid of square
cells, each of which is in one of two possible cellss, alive or dead, (or populated and unpopulated,
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

// Icons for Cell representations. Remember, the unicode character is sometimes
// two characters wide. We handle DEAD_ICON specially in the code we can still
// use a `char` datatype.
const KILLED_ICON: char = '💔';
const BORN_ICON: char = '💚';
const SURVIVED_ICON: char = '💙';
const DEAD_ICON: char = 'b';

// TODO: Make arguments.
const WIDTH: usize = 40;
const HEIGHT: usize = 40;


pub struct Generation {
    cells: [[char; WIDTH]; HEIGHT],
    n: u64,
    born: u64,
    killed: u64,
    survived: u64,
}

impl Generation {

    // Initialize a new Generation and its stats.
    pub fn new() -> Self {
        Self {
            cells: [[DEAD_ICON; WIDTH]; HEIGHT],
            n: 0,
            born: 0,
            killed: 0,
            survived: 0,
        }
    }

    pub fn seed(&mut self) {
        /*
        for i in 0..HEIGHT {
            self.cells[i][0] = BORN_ICON;
            self.cells[i][WIDTH-1] = BORN_ICON;
        }
        for i in 0..WIDTH {
            self.cells[0][i] = BORN_ICON;
            self.cells[HEIGHT-1][i] = BORN_ICON;
            //self.cells[HEIGHT-2][i] = KILLED_ICON;
        }
        */

        // THIS ONE TOO
        self.cells[0][0] = BORN_ICON;
        self.cells[0][WIDTH-1] = BORN_ICON;
        self.cells[HEIGHT-1][0] = BORN_ICON;
        self.cells[HEIGHT-1][WIDTH-1] = BORN_ICON;
        self.cells[10][10] = BORN_ICON;
        self.cells[11][10] = BORN_ICON;
        self.cells[12][10] = BORN_ICON;
        self.cells[20][20] = BORN_ICON;
        self.cells[20][21] = BORN_ICON;
        self.cells[20][22] = BORN_ICON;
        self.cells[17][22] = BORN_ICON;
        self.cells[18][23] = BORN_ICON;
        self.cells[19][24] = BORN_ICON;
        self.cells[20][25] = BORN_ICON;
        self.cells[17][5] = BORN_ICON;
        self.cells[18][5] = BORN_ICON;
        self.cells[19][5] = BORN_ICON;
        self.cells[20][5] = BORN_ICON;
        self.cells[17][7] = BORN_ICON;
        self.cells[18][8] = BORN_ICON;
        self.cells[19][9] = BORN_ICON;
        self.cells[20][9] = BORN_ICON;
        self.cells[20][10] = BORN_ICON;
        self.cells[20][11] = BORN_ICON;
        self.cells[21][11] = BORN_ICON;
        self.cells[22][11] = BORN_ICON;
        self.cells[23][11] = BORN_ICON;
        /*
        self.cells[30][11] = BORN_ICON;
        self.cells[30][12] = BORN_ICON;
        self.cells[31][12] = BORN_ICON;
        self.cells[31][13] = BORN_ICON;
        self.cells[32][14] = BORN_ICON;
        self.cells[33][15] = BORN_ICON;
        self.cells[34][16] = BORN_ICON;
        self.cells[35][17] = BORN_ICON;
        */

        self.cells[HEIGHT-1][WIDTH-1] = BORN_ICON;
        self.cells[HEIGHT-1][WIDTH-1] = BORN_ICON;
        self.cells[HEIGHT-1][WIDTH-1] = BORN_ICON;

        for i in HEIGHT-12..HEIGHT-1 {
            for j in WIDTH-6..WIDTH-1 {
                self.cells[i][j] = BORN_ICON;
            }
        }

        for i in 0..8 {
            for j in 0..12 {
                self.cells[i][j] = BORN_ICON;
            }
        }

        /*
        for i in 10..25 {
            for j in 30..40 {
                self.cells[i][j] = BORN_ICON;
            }
        }
        */
    }

    // Create a new Generation based off the current Generation.
    pub fn tick(&mut self) -> Generation {
        let mut f_generation = Generation::new();

        for (i, row) in self.cells.iter().enumerate() {
            for (j, _col) in row.iter().enumerate() {

                let n = self.neighbor_count(i, j);

                if self.is_cell_alive(i, j) {
                    if n < 2 || n > 3 {
                        f_generation.cells[i][j] = KILLED_ICON;
                        f_generation.killed += 1;
                    } else {
                        f_generation.cells[i][j] = SURVIVED_ICON;
                        f_generation.survived += 1;
                    }
                } else {  // is_dead, essentially :)
                    if n == 3 {
                        f_generation.cells[i][j] = BORN_ICON;
                        f_generation.born += 1
                    }
                }
            }
        }

        f_generation.n = self.n + 1;

        return f_generation
    }

    // Helper for finding out if a specific cell is alive.
    pub fn is_cell_alive(&self, i: usize, j: usize) -> bool {
        match self.cells[i][j] {
            BORN_ICON => true,
            SURVIVED_ICON => true,
            _ => false,
        }
    }

    // Count all neighboring cells that are alive.
    pub fn neighbor_count(&self, i: usize, j: usize) -> i8 {
        let l = self.cells.len() - 1;
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

    // Helper for printing all Cells of a Generation.
    pub fn print(&self) {
        // Clears the screen.
        //print!("{}[2J", 27 as char);
        for row in self.cells.iter() {
            for col in row.iter() {
                match col {
                    'b' => print!("  "),  // needed since emoji's used are 2 char wide
                    _ => print!("{}", col),
                }
            }
            println!();
        }

        // Generation stats.
        println!("\nGeneration {: <4} // Killed {: <3} // Survived {: <3} // Born {: <3}",
                 &self.n, &self.killed, &self.survived, &self.born);
    }

}

// Keep track of Generation stats so we can preempt further Generations.
pub struct GenerationLifeCycle {
    n: u8,
    items: [u64; 3],
}

// Game of Life state.
pub struct GoL {
    width: usize,
    height: usize,
    tick_rate: u64,
    generation: Generation,
}

impl GoL {

    // Initialize the first (aka seed) Generation.
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

    // "Start" the GoL by generating a new generation and printing it.
    pub fn start(&mut self) {
        let mut glc = GenerationLifeCycle {
            n: 0,
            items: [0, 0, 0],
        };
        loop {
            self.generation.print();
            self.generation = self.generation.tick();

            if self.has_stabilized(&mut glc) {
                println!("\nGenerations have stabalized!\n");
                break;
            }

            thread::sleep(time::Duration::from_millis(self.tick_rate));
        }
    }

    // If the generations haven't changed in several iterations, then consider
    // the Cells stabilized.
    fn has_stabilized(&self, glc: &mut GenerationLifeCycle) -> bool {
        let generation_items = [
            self.generation.born,
            self.generation.killed,
            self.generation.survived,
        ];
        if generation_items == glc.items {
            glc.n += 1;
        } else {
            glc.n = 1;
            glc.items = generation_items
        }

        glc.n > 6
    }

}
