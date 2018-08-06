mod gol;

const WIDTH: usize = 30;
const HEIGHT: usize = 40;
const TICK_RATE: u64 = 100;


fn main() {
    let mut g = gol::GoL::new(WIDTH, HEIGHT, TICK_RATE);
    loop {
        // Print the GoL state.
        g.print();

        // Spawn another generation.
        g.tick();
    }
}
