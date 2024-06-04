use crate::{cards::deck::Deck, player::Player};
use super::super::{
    actions::*,
    phases::*
};

struct BasicRummyState {
    deck: Deck,
    players: Vec<Player>,
    round: usize,
    cur_player: usize,
    score: Score
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

        if card_i > player_cards.len() {
            return TransitionResult::Error((
                self,
                format!("card_i ({}) is greater than player's hand size ({})", card_i, player_cards.len())
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

    fn to_next_player(mut self) -> Self::SelfInDrawPhase {
        let state = self.state;
        
        state.cur_player = (state.cur_player + 1) % state.players.len();
        while (!state.players[state.cur_player].active) { // find the next active player
            state.cur_player += 1;
        }

        BasicRummy {
            phase: DrawPhase { has_drawn: false },
            state
        }
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
        todo!()
    }

    fn quit_player(self, player_i: usize) 
    -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String> 
    {
        todo!()
    }
    
    fn quit_current_player(self) -> Self::SelfInDrawPhase {
        todo!()
    }
}