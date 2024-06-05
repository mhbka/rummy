use crate::{
    player::{self, Player},
    cards::{
        card, 
        deck::Deck, 
        meld::{
            Meld, 
            Meldable, 
            Run, 
            Set
        }
    }
};
use super::super::{
    actions::*,
    phases::*
};

struct BasicRummyState {
    deck: Deck,
    players: Vec<Player>,
    cur_round: usize,
    cur_player: usize
}

/// A basic game of Rummy, following the rules/behaviour described [here](https://en.wikipedia.org/wiki/Rummy).
pub struct BasicRummy<P: GamePhase> {
    phase: P,
    state: BasicRummyState
}


impl DrawActions for BasicRummy<DrawPhase> {
    type SelfInPlayPhase = BasicRummy<PlayPhase>;

    fn draw_stock(&mut self) -> Result<(), String> {
        let card = &mut self.state.deck.draw(1)?;

        self.state
            .players[self.state.cur_player]
            .cards
            .append(card);
        
        if self.state
            .deck
            .stock().len() == 0 {
                self.state.deck.turnover_discarded();
            }

        self.phase.has_drawn = true;
        
        Ok(())
    }

    fn draw_discard_pile(&mut self, amount: Option<usize>) -> Result<(), String> {
        self.state
            .players[self.state.cur_player]
            .cards
            .append(
                &mut self.state.deck.draw_discard_pile(amount)?
            );

        self.phase.has_drawn = true;

        Ok(())
    }

    fn to_play(mut self) -> Self::SelfInPlayPhase {
        if !self.phase.has_drawn {
            self.draw_stock()
                .expect("Drawing 1 card should always be OK");
        }  
        BasicRummy {
            phase: PlayPhase { play_count: 0 },
            state: self.state
        }
    }
}


impl PlayActions for BasicRummy<PlayPhase> {
    type SelfInDiscardPhase = BasicRummy<DiscardPhase>;
    type SelfInRoundEndPhase = BasicRummy<RoundEndPhase>;

    fn form_meld(mut self, card_indices: Vec<usize>) 
    -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String>
    {
        if card_indices.len() < 3 {
            return TransitionResult::Error((
                self,
                "card_indices has less than 3 elements; need at least 3 for a meld".to_owned()
            ));
        }

        let player = &mut self.state.players[self.state.cur_player];
        let mut meld_cards = Vec::new();
        
        for i in card_indices {
            if i > player.cards.len() {
                return TransitionResult::Error((
                    self,
                    format!("An index in card_indices ({i}) is greater than player's hand's size")
                ))
            }
            else {
                meld_cards.push(player.cards[i].clone());
            }
        }

        if let Ok(meld) = Meld::new(&mut meld_cards) {
            player.melds.push(meld);
        }
        else {
            return TransitionResult::Error((
                self,
                "Cards do not form a valid set or run".to_owned()
            ))
        }

        TransitionResult::Next(self)
    }

    fn layoff_card(mut self, card_i: usize, target_player_i: usize, target_meld_i: usize)
    -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String>
    {
        let err_string;

        // check that all indices are valid first
        if card_i >= self.state.players[self.state.cur_player].cards.len() {
            err_string = "card_i is greater than current player's hand size";
        } 
        else if target_player_i >= self.state.players.len() {
            err_string = "target_player_i is greater than number of players";
        } 
        else if !self.state.players[target_player_i].active {
            err_string = "Target player is not active";
        } 
        else if target_meld_i >= self.state.players[target_player_i].melds.len() {
            err_string = "target_meld_i is greater than target player's number of melds";
        } 
        else {
            let card = self.state.players[self.state.cur_player]
                .cards
                .remove(card_i);

            let meld = &mut self.state.players[target_player_i].melds[target_meld_i];

            match meld.layoff_card(card) {
                Ok(_) => return TransitionResult::Next(self),
                Err(card) => {
                    self.state.players[self.state.cur_player]
                        .cards
                        .insert(card_i, card);
                    err_string = "Layoff was not valid";
                }
            }
        }

        TransitionResult::Error((
            self, 
            err_string.to_owned()
        ))
    }

    fn to_discard(self) -> Self::SelfInDiscardPhase {
        BasicRummy {
            phase: DiscardPhase { has_discarded: false },
            state: self.state
        }
    }
}


impl DiscardActions for BasicRummy<DiscardPhase> {
    type SelfInDrawPhase = BasicRummy<DrawPhase>;
    type SelfInRoundEndPhase = BasicRummy<RoundEndPhase>;

    fn discard(mut self, card_i: usize) 
    -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String> 
    {
        if self.phase.has_discarded {
            return TransitionResult::Error((
                self,
                "Player has already discarded a card".to_owned()
            ));
        }

        let player_cards = &mut self.state
            .players[self.state.cur_player]
            .cards;

        let no_player_cards = player_cards.len();

        if card_i > no_player_cards {
            return TransitionResult::Error((
                self,
                format!("card_i ({}) is greater than player's hand size ({})", card_i, no_player_cards)
                ));
        }

        let card = player_cards.remove(card_i);
        self.state
            .deck
            .add_to_discard_pile(&mut vec![card]);

        if player_cards.len() == 0 {
            TransitionResult::End(
                BasicRummy {
                    phase: RoundEndPhase { has_scored_round: false },
                    state: self.state
                }
            )
        }
        else {
            self.phase.has_discarded = true;
            TransitionResult::Next(
                self
            )
        }
    }

    fn to_next_player(mut self)
    -> TransitionResult<Self::SelfInDrawPhase, Self::SelfInRoundEndPhase, Self, String> 
    {   
        // automatically discard the first card if discard hasn't been called yet,
        if !self.phase.has_discarded { 
            match self.discard(0) {
                TransitionResult::Next(s) => self = s,
                TransitionResult::End(e) => return TransitionResult::End(e),
                TransitionResult::Error(_) => unreachable!() // discarding first card should never error
            }
        }

        let mut state = self.state;
        
        state.cur_player = (state.cur_player + 1) % state.players.len();
        while !state.players[state.cur_player].active { // find the next active player
            state.cur_player = (state.cur_player + 1) % state.players.len();
        }

        TransitionResult::Next(
            BasicRummy {
                phase: DrawPhase { has_drawn: false },
                state
            }
        )
    }
}


impl RoundEndActions for BasicRummy<RoundEndPhase> {
    type SelfInDrawPhase = BasicRummy<DrawPhase>;

    fn calculate_score(&mut self) {
        self.phase.has_scored_round = true;

        todo!()
    }

    fn to_next_round(mut self) -> Self::SelfInDrawPhase {
        if !self.phase.has_scored_round {
            self.calculate_score();
        }

        let mut state = self.state;
        state.deck.reset();

        for player in &mut state.players {
            player.melds.clear();
            player.cards.clear();
            if player.joined_in_round == state.cur_round {
                player.active = true;
            }
        }

        state.cur_round += 1;

        BasicRummy {
            phase: DrawPhase { has_drawn: false },
            state
        }
    }
}


impl GameEndActions for BasicRummy<GameEndPhase> {

}


impl<P: GamePhase + PlayablePhase> PlayableActions for BasicRummy<P> {
    type SelfInRoundEndPhase = BasicRummy<RoundEndPhase>;
    type SelfInDrawPhase = BasicRummy<DrawPhase>;

    fn add_player(&mut self, player_id: usize, index: Option<usize>) {
        let player = Player::new(player_id, false, self.state.cur_round);

        if index.is_none() || index.is_some_and(|i| i > self.state.players.len()) {
            self.state.players.push(player);
        }
        else {
            self.state.players.insert(index.unwrap(), player);
        }
    }

    fn quit_player(mut self, player_i: usize) 
    -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String> 
    {
        if player_i == self.state.cur_player || player_i > self.state.players.len() {
            return TransitionResult::Error((
                self,
                format!("player_i {player_i} was either the current player, or greater than number of players")
            ));
        }

        self.state.players[self.state.cur_player].active = false;

        if self.state.players.iter().all(|p| !p.active) { // end round if 
            return TransitionResult::End(
                BasicRummy {
                    phase: RoundEndPhase { has_scored_round: false},
                    state: self.state
                }
            );
        }
        else {
            return TransitionResult::Next(
                self
            );
        }
    }
    
    fn quit_current_player(mut self) -> Self::SelfInDrawPhase {
        self.state.players[self.state.cur_player].active = false;

        let mut state = self.state;
        
        state.cur_player = (state.cur_player + 1) % state.players.len();
        while !state.players[state.cur_player].active { // find the next active player
            state.cur_player = (state.cur_player + 1) % state.players.len();
        }

        BasicRummy {
            phase: DrawPhase { has_drawn: true },
            state
        }
    }

    fn move_card_in_hand(&mut self, player_i: usize, old_pos: usize, mut new_pos: usize) -> Result<(), String> {
        if player_i > self.state.players.len() {
            return Err(format!("player_i {player_i} is greater than number of players"));
        }
        
        let player_hand = &mut self.state.players[player_i].cards;
        if old_pos > player_hand.len() {
            return Err(format!("old_pos {old_pos} is greater than the player's hand's size"));
        }

        if new_pos > player_hand.len() {
            new_pos = player_hand.len() - 1;
        }

        let card = player_hand.remove(old_pos);
        player_hand.insert(new_pos - 1, card); 

        Ok(())
    }
}