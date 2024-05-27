/// Trait indicating a game phase.
pub(crate) trait GamePhase {}

/// Trait indicating a phase where the game can still be played.
pub(crate) trait PlayablePhase {}

// GamePhase options.
pub(crate) struct DrawPhase {
    pub(super) has_drawn: bool
}
pub(crate) struct PlayPhase {
    pub(super) move_count: usize
}
pub(crate) struct DiscardPhase {
    pub(super) has_discarded: usize
}
pub(crate) struct RoundEndPhase {
    pub(super) has_scored_round: bool
}
pub(crate) struct GameEndPhase {
    // no state needed, game has ended
}

// Mark these as GamePhases.
impl GamePhase for DrawPhase {}
impl GamePhase for PlayPhase {}
impl GamePhase for DiscardPhase {}
impl GamePhase for RoundEndPhase {}
impl GamePhase for GameEndPhase {}

// Mark these as PlayablePhases (for PlayableActions).
impl PlayablePhase for DrawPhase {}
impl PlayablePhase for PlayPhase {}
impl PlayablePhase for DiscardPhase {}
impl PlayablePhase for RoundEndPhase {}

pub(crate) trait HasGamePhase<P1: GamePhase> {
    type NextGamePhase<P2: GamePhase>: HasGamePhase<P2, NextGamePhase<P1> = Self>;
}