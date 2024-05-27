use super::{phases::{
    DiscardPhase, DrawPhase, GameEndPhase, GamePhase, HasGamePhase, PlayPhase, PlayablePhase, RoundEndPhase
}, Game};

/// The result of a game phase transition:
/// - Next: A `Game` with gamephase `P`, the logical next phase (ie Draw -> Play, Play -> Discard).
/// - End: A `Game` with gamephase `RoundEndPhase` (ie, the round ends due to some game condition).
pub(crate) enum NextPhase<G: HasGamePhase<P>, P: GamePhase> {
    Next(<G as HasGamePhase<P>>::NextGamePhase<P>),
    End(<G as HasGamePhase<P>>::NextGamePhase<RoundEndPhase>),
}

/// Trait for transitioning from one gamephase to another.
/// 
/// If the round has logically ended, the result is `NextPhase::End`.
/// Otherwise, the game continues to the next phase as `NextPhase::Next`.
/// 
/// As it is logically infallible, there should be some default behaviour if the game 
/// currently cannot transition logically.
/// 
/// For example, if `next()` is called during DrawPhase, but the player hasn't drawn yet,
/// a card should automatically be drawn so the transition can still occur.
pub(crate) trait PhaseTransition<G, PCur, PNext>
where
    PCur: GamePhase,
    PNext: GamePhase,
    G: HasGamePhase<PCur> + HasGamePhase<PNext>,
    Self: HasGamePhase<PCur>,
{
    fn next(self) -> NextPhase<G, PNext>;
}

/// Supertrait which encompasses the possible phase transitions in a game of Rummy.
pub(crate) trait GamePhaseTransitions<G, P>: 
    PhaseTransition<G, RoundEndPhase, DrawPhase>
    + PhaseTransition<G, DrawPhase, PlayPhase>
    + PhaseTransition<G, PlayPhase, DiscardPhase>
    + PhaseTransition<G, DiscardPhase, DrawPhase>
where
    G: HasGamePhase<DrawPhase>
    + HasGamePhase<PlayPhase>
    + HasGamePhase<DiscardPhase>
    + HasGamePhase<RoundEndPhase>
    + HasGamePhase<GameEndPhase>
    {}