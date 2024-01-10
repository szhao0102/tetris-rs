mod mino;
mod game;

use game::Game;
fn main() {
    let mut g = Game::init();
    g.render();
}




