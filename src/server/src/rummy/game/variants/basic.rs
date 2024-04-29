use crate::rummy::player::{self, Player};
use crate::rummy::game::state::GameState;
use crate::rummy::cards::{
    card::Card,
    deck::{Deck, DeckConfig}
};
use super::super::traits::{
    GameInit,
    GameActions,
    GameAdmin
};


/// Holds customizable settings for a basic Rummy game.
pub struct BasicConfig {
    pub deck_config: DeckConfig
}


/// A basic Rummy game;
/// follows the implementation detailed [here](https://en.wikipedia.org/wiki/Rummy#Basic_rummy).
pub struct BasicRummy {
    pub(super) config: BasicConfig,
    pub(super) state: GameState,
    pub(super) deck: Deck,
    pub(super) players: Vec<Player>
}

impl BasicRummy {
    /// Get the amount of cards to deal given the player and pack count;
    /// follows the [Wiki rules](https://en.wikipedia.org/wiki/Rummy#Basic_rummy).
    /// 
    /// If the counts don't follow the rules, an `Err` is returned.
    fn get_deal_count(player_count: usize, pack_count: usize) -> Result<usize, String> {
        let deal_count = match (player_count, pack_count) {
            (2, 1) => 10,
            (3..=10, 1) => 7,
            (3..=10, _) => 10,
            (6, 1) => 6,
            (6, _) => 10,
            (7, 2) => 10,
            _ => {
                return Err(format!(
                    "Unallowed player count ({player_count}) and pack count ({pack_count})"
                ));
            }
        };
    
        Ok(deal_count)
    }

    /// Gets the number of currently active players.
    fn get_active_players(&self) -> usize {
        self.players
            .into_iter()
            .fold(0, |acc, p| acc + p.active as usize)
    }
}


impl GameInit for BasicRummy {
    type Config = BasicConfig;
    
    /// Create a basic Rummy game. Note the following constraints:
    /// - 2-7 players only
    /// - 3-6 players may choose between using 1 or 2 decks
    fn new(player_ids: Vec<usize>, config: Self::Config) -> Result<Self, String> {
        let pack_count = config.deck_config.pack_count;
        let player_count = player_ids.len();
        BasicRummy::get_deal_count(player_count, pack_count)?; // TODO: is it a good idea to use this for validation too?

        let state = GameState::new();
        let deck = Deck::new(config.deck_config)?;
        let players = player_ids
            .iter()
            .map(|&id| Player::new(id))
            .collect();

        Ok(
            BasicRummy { config, state, deck, players }
        )
    }

    fn init_round(&mut self) -> Result<(), String> {
        self.players
            .iter()
            .for_each(|player| player.reset());

        let pack_count = self.config.deck_config.pack_count;
        let player_count = self.get_active_players();
        let deal_count = BasicRummy::get_deal_count(player_count, pack_count)?;

        for player in &self.players {
            let mut cards = self.deck.draw(deal_count).unwrap();
            player.cards.append(&mut cards);
        }

        Ok(())
    }
}

impl GameActions for BasicRummy {
    fn draw_deck(&mut self) -> Result<(), String> {
        todo!()
    }

    fn draw_discard_pile(&mut self) -> Result<(), String> {
        todo!()
    }

    fn form_meld(&mut self, indices: Vec<usize>) -> Result<(), String> {
        todo!()
    }

    fn layoff_card(
        &mut self, 
        card_index: usize, 
        target_player_index: usize, 
        target_meld_index: usize) -> Result<(), String> {
        todo!()
    }

    fn discard_card(&mut self, card_index: usize) -> Result<(), String> {
        todo!()
    }
}

impl GameAdmin for BasicRummy {
    fn player_join(&mut self, player_id: usize, index: Option<usize>) -> Result<(), String> {
        todo!()
    }

    fn player_quit(&mut self, index: usize) -> Result<(), String> {
        todo!()
    }

    fn calculate_score(&mut self) -> Result<(), String> {
        todo!()
    }
}
