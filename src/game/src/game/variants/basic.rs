use crate::{cards::deck::Deck, player::Player};

use super::super::{
    actions::*,
    phases::*
};

pub(crate) struct BasicRummy<P: GamePhase> {
    phase: P,
    players: Vec<Player>,
    deck: Deck
}

impl DrawActions for BasicRummy<DrawPhase> {
    type SelfInPlayPhase = BasicRummy<PlayPhase>;

    type SelfInRoundEndPhase = BasicRummy<RoundEndPhase>;

    fn draw_stock(&mut self) -> Result<(), String> {
        todo!()
    }

    fn draw_discard_pile(&mut self) -> Result<(), String> {
        todo!()
    }

    fn to_play(self) -> Result<Self::SelfInPlayPhase, Self::SelfInRoundEndPhase> {
        todo!()
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
        todo!()
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