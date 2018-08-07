mod gol;

const TICK_RATE: u64 = 100;


fn main() {
    let mut g = gol::GoL::new(TICK_RATE);
    g.start();
}
