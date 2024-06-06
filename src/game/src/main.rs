pub mod cards;
pub mod game;
pub mod player;

use game::{actions::{DrawActions, RoundEndActions}, variants::standard::StandardRummyGame};

fn main() {
    let mut draw_phase = StandardRummyGame::quickstart(vec![1, 2, 3, 4])
        .to_next_round();
        
    draw_phase.draw_stock();

    let mut play_phase = draw_phase.to_play_phase();
}
