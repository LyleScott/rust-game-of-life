mod gol;

const TICK_RATE: u64 = 100;


fn main() {
    gol::GoL::new(TICK_RATE).start();
}
