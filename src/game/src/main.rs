pub mod cards;
pub mod game;
pub mod player;

use game::{actions::{DrawActions, PlayActions, RoundEndActions}, variants::standard::{StandardRummyGame, StandardRummyState}};

fn main() {
    let player_ids = vec![1, 2, 3, 4];

    let mut game = StandardRummyGame::quickstart(player_ids).to_next_round();
    game.draw_stock();

    let mut game = game.to_play_phase();
    print_state(game.view_state());
}

fn print_state(state: &StandardRummyState) {

}
