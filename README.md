# Conway's Game of Life

... written in Rust. This is something I used to learn more Rust concepts.

* 💚 = cell was born in the generation
* 💙 = cell survived the generation
* 💔 = cell was killed in the generation

![Screenshot](/screenshot.jpg)

## Running the Game

```
cargo build
cargo run
```

## Rules

Any live cell with fewer than two live neighbors dies, as if by under population.
Any live cell with two or three live neighbors lives on to the next generation.
Any live cell with more than three live neighbors dies, as if by overpopulation.
Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

The initial pattern constitutes the seed of the system. The first generation is created by applying
the above rules simultaneously to every cell in the seed; births and deaths occur simultaneously,
and the discrete moment at which this happens is sometimes called a tick. Each generation is a pure
function of the preceding one. The rules continue to be applied repeatedly to create further
generations.

In this implementation, the generations stop and the "game" ends when no more cells are
born/killed.
