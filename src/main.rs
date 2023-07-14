mod comms;
mod game;
mod object_types;

fn main() {
    let mut game = game::Game::new();

    // Continue reading until the game ends.
    while game.read_next_turn() {
        game.respond_to_turn();
    }
}
