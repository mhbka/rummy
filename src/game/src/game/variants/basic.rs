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


pub struct BasicRummy<P: GamePhase> {
    phase: P,
    state: BasicRummyState
}


impl DrawActions for BasicRummy<DrawPhase> {
    type SelfInPlayPhase = BasicRummy<PlayPhase>;
    type SelfInRoundEndPhase = BasicRummy<RoundEndPhase>;

    fn draw_stock(&mut self) -> Result<(), String> {
        self.state
            .players[self.state.cur_player]
            .cards
            .append(
                &mut self.state.deck.draw(1)?
            );
        
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

    fn to_play(mut self) -> Result<Self::SelfInPlayPhase, Self::SelfInRoundEndPhase> {
        if !self.phase.has_drawn {
            self.draw_stock()
                .expect("Drawing 1 card should always be OK");
        }  

        // if no players are active, return Err(Self::SelfInRoundEndPhase)
        if self.state
            .players 
            .iter()
            .all(|p| !p.active)
        {
            return Err(
                BasicRummy {
                    phase: RoundEndPhase { has_scored_round: false },
                    state: self.state
                }
            );
        }

        Ok(
            BasicRummy {
                phase: PlayPhase { play_count: 0 },
                state: self.state
            }
        )
    }
}

impl PlayActions for BasicRummy<PlayPhase> {
    type SelfInDiscardPhase = BasicRummy<DiscardPhase>;
    type SelfInRoundEndPhase = BasicRummy<RoundEndPhase>;

    fn form_meld(&mut self, card_indices: Vec<usize>) -> Result<(), String> {
        todo!()
    }

    fn layoff_card(&mut self, card_i: usize, target_player_i: usize, target_meld_i: usize) -> Result<(), String> {
        todo!()
    }

    fn to_discard(self) -> Result<Self::SelfInDiscardPhase, Self::SelfInRoundEndPhase> {
        todo!()
    }
}

impl DiscardActions for BasicRummy<DiscardPhase> {
    type SelfInDrawPhase = BasicRummy<DrawPhase>;
    type SelfInRoundEndPhase = BasicRummy<RoundEndPhase>;

    fn discard(&mut self, card_i: usize) -> Result<(), String> {
        let player_cards = &mut self.state
            .players[self.state.cur_player]
            .cards;

        if card_i > player_cards.len() {
            return Err(format!("card_i ({}) is greater than player's hand size ({})", card_i, player_cards.len()));
        }

        let card = player_cards.remove(card_i);

        self.state
            .deck
            .add_to_discard_pile(&mut vec![card]);

        Ok(())
    }

    fn to_next_player(self) -> Result<Self::SelfInDrawPhase, Self::SelfInRoundEndPhase> {
        todo!()
    }
}

impl RoundEndActions for BasicRummy<RoundEndPhase> {
    type SelfInPlayPhase = BasicRummy<PlayPhase>;

    fn calculate_score(&mut self) -> Result<(), String> {
        todo!()
    }

    fn to_next_round(self) -> Self::SelfInPlayPhase {
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

    fn quit_player(self, player_i: usize) -> Result<Self, Self::SelfInRoundEndPhase> {
        todo!()
    }
    
    fn quit_current_player(self) -> Self::SelfInDrawPhase {
        todo!()
    }
}