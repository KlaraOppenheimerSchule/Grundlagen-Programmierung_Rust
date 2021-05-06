mod logic;
use logic::Game;

fn main() {
    let mut my_game = Game::new((3, 3));
    my_game.start();
}
