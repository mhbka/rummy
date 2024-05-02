use crate::rummy::player::{self, Player};
use crate::rummy::game::state::{GamePhase, GameState};
use crate::rummy::cards::{
    meld::{Meld, Set, Run}
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

    /// Verifies that the current gamephase matches the intended one.
    fn verify_gamephase(&self, intended_phase: GamePhase) -> Result<(), String> {
        if self.state.phase == intended_phase { return Ok(()); }
        return Err(format!("Required game phase: {:?} (actual: {:?})", intended_phase, self.state.phase));
    }

    /// Returns the current player.
    fn get_current_player(&self) -> &Player {
        &self.players[self.state.player_index]
    }
}


impl GameInit for BasicRummy {
    type Config = BasicConfig;
    
    /// Create a basic Rummy game. Note the following constraints:
    /// - 2-7 players only
    /// - 3-6 players may choose between using 1 or 2 decks
    /// 
    /// Breaking a constraint in `config` will return an `Err`.
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
        self.verify_gamephase(GamePhase::RoundEnd)?;

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
        self.verify_gamephase(GamePhase::PlayerPlays)?;

        let mut card = self.deck.draw(1).unwrap(); // drawing 1 should always be OK
        let player = &self.players[self.state.player_index];
        player.cards.append(&mut card);
        Ok(())
    }

    fn draw_discard_pile(&mut self) -> Result<(), String> {
        self.verify_gamephase(GamePhase::PlayerPlays)?;

        let mut card = self.deck.draw_discard_pile(Some(1)).unwrap(); // drawing 1 should always be OK
        let player = self.get_current_player();
        player.cards.append(&mut card);
        Ok(())
    }

    fn form_meld(&mut self, indices: Vec<usize>) -> Result<(), String> {
        self.verify_gamephase(GamePhase::PlayerPlays)?;

        let cards = self.get_current_player().cards
            .iter()
            .enumerate()
            .filter(|(idx, _)| indices.contains(idx))
            .map(|(_, &card)| card)
            .collect();
    }

    fn layoff_card(
        &mut self, 
        card_index: usize, 
        target_player_index: usize, 
        target_meld_index: usize) 
        -> Result<(), String> 
    {
        self.verify_gamephase(GamePhase::PlayerPlays)?;
    }

    fn discard_card(&mut self, card_index: usize) -> Result<(), String> {
        self.verify_gamephase(GamePhase::PlayerDraw)?;
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
        self.verify_gamephase(GamePhase::GameEnd)?;
    }
}
