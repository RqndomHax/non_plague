mod entities;
mod game;
mod modifiers;
mod traits;

use game::*;

fn main() {
    let mut game = Game::init();

    game.test_entity();
    game.test_entity();
    game.test_entity();
}
