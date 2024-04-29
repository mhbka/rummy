use crate::rummy::player::Player;
use crate::rummy::game::state::GameState;
use crate::rummy::cards::{
    card::Card,
    deck::Deck
};
use super::super::traits::{
    GameInit,
    GameActions,
    GameAdmin,
    GameScoring 
};



/// Holds customizable settings for a basic Rummy game.
pub struct BasicConfig {
    
}


/// A basic Rummy game;
/// follows the implementation detailed [here](https://en.wikipedia.org/wiki/Rummy#Basic_rummy).
pub struct BasicRummy {
    pub(super) state: GameState,
    pub(super) deck: Deck,
    pub(super) players: Vec<Player>,
    pub(super) discard_pile: Vec<Card>
}

impl GameInit for BasicRummy {
    type Config = BasicConfig;
    
    fn new(player_ids: Vec<usize>, config: Self::Config) -> Self {
        todo!()
    }

    fn init_round(&mut self) -> Result<(), crate::rummy::game::error::GameError> {
        todo!()
    }
}

impl GameActions for BasicRummy {
    fn draw_deck(&mut self) -> Result<(), crate::rummy::game::error::GameError> {
        todo!()
    }

    fn draw_discard_pile(&mut self) -> Result<(), crate::rummy::game::error::GameError> {
        todo!()
    }

    fn form_meld(&mut self, indices: Vec<usize>) -> Result<(), crate::rummy::game::error::GameError> {
        todo!()
    }

    fn layoff_card(
        &mut self, 
        card_index: usize, 
        target_player_index: usize, 
        target_meld_index: usize) -> Result<(), crate::rummy::game::error::GameError> {
        todo!()
    }

    fn discard_card(&mut self, card_index: usize) -> Result<(), crate::rummy::game::error::GameError> {
        todo!()
    }
}
