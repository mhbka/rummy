use actions::*;

pub mod actions;
pub mod phases;
pub mod error;
pub mod variants;
pub mod state;

pub trait Game {
    type InDrawPhase: DrawActions;
    type InPlayPhase: PlayActions;
    type InDiscardPhase: DiscardActions;
    type InRoundEndPhase: RoundEndActions;
    type InGameEndPhase: GameEndActions;
}

