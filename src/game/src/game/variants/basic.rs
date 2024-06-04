use crate::{cards::deck::Deck, player::{self, Player}};
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
    -> TransitionResult<Self::SelfInDiscardPhase, Self::SelfInRoundEndPhase, Self, String>
    {
        todo!()
    }

    fn layoff_card(mut self, card_i: usize, target_player_i: usize, target_meld_i: usize)
    -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String>
    {
        todo!()
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
                TransitionResult::Error(err) => unreachable!() // discarding first card should never error
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

    fn calculate_score(&mut self) -> Result<(), String> {
        todo!()
    }

    fn to_next_round(self) -> Self::SelfInDrawPhase {
        todo!()
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