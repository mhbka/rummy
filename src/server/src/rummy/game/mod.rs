pub mod state;
pub mod error;
pub mod traits;

use self::state::GameState;
use super::{
    cards::{
        card::Card, 
        deck::Deck
    }, 
    player::Player
};

/// The public interface of the game.
pub struct Game {
    pub(super) state: GameState,
    pub(super) deck: Deck,
    pub(super) players: Vec<Player>,
    pub(super) discard_pile: Vec<Card>
}

