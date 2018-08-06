mod gol;

const WIDTH: usize = 30;
const HEIGHT: usize = 40;
const TICK_RATE: u64 = 200;


fn main() {
    let mut g = gol::Game::new(WIDTH, HEIGHT, TICK_RATE);

    loop {
        print!("{}[2J", 27 as char);
        g.print();
        g.tick();
        //break;
    }
}
