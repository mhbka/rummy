use super::phases::{
    GamePhase,
    PlayablePhase,
    DrawPhase,
    PlayPhase,
    DiscardPhase,
    RoundEndPhase,
    GameEndPhase,
    HasGamePhase
};


/// The result of a game phase transition:
/// - Next: A `Game` with gamephase `P`, the logical next phase (ie Draw -> Play, Play -> Discard).
/// - End: A `Game` with gamephase `RoundEndPhase` (ie, the round ends due to some game condition).
/// 
/// **NOTE**: Currently this requires a user to fill in `P1`, but when `G` is `Self`,
/// `P1` is already implicitly defined, which makes the API here confusing.
/// 
/// TODO: work out how to remove the need for P1 to be defined
pub(crate) enum TransitionResult<G, P1, P2>
where
    G: HasGamePhase<P1>,
    P1: GamePhase,
    P2: GamePhase
{
    Next(G::SelfHasGamePhase<P2>),
    End(G::SelfHasGamePhase<RoundEndPhase>),
}


/// A supertrait encompassing all actions.
/// TODO: define trait bound on associated type here? or in HasGamePhase
pub(crate) trait GameActions:
    DrawActions
    + PlayActions 
    + DiscardActions
    + RoundEndActions
    + GameEndActions
{}

/// Trait for actions during DrawPhase.
pub(crate) trait DrawActions where Self: HasGamePhase<DrawPhase> + Sized {
    /// Draw from the stock for the current player.
    fn draw_stock(&mut self) -> Result<(), String>;

    /// Draw from the discard pile for the current player.
    fn draw_discard_pile(&mut self) -> Result<(), String>;

    /// Transition to next state where the current player can make plays.
    /// 
    /// **NOTE**: Ensure any required actions are taken by the time/during this function call,
    /// as it is infallible.
    fn to_play(self) -> TransitionResult<Self, DrawPhase, PlayPhase>;
}

/// Trait for actions during PlayPhase.
pub(crate) trait PlayActions where Self: HasGamePhase<PlayPhase> + Sized {
    /// Form a meld from a Vec of indices,
    /// referring to cards in the current player's hand.
    fn form_meld(&mut self, card_indices: Vec<usize>) -> Result<(), String>;

    /// Layoff `card_i` card in the current player's hand,
    /// to `target_player_i` player's `target_meld_i` meld.
    fn layoff_card(&mut self, card_i: usize, target_player_i: usize, target_meld_i: usize) -> Result<(), String>;

    /// Transition to the next state where the current player can discard.
    /// 
    /// **NOTE**: Ensure any required actions are taken by the time/during this function call,
    /// as it is infallible.
    fn to_discard(self) -> TransitionResult<Self, PlayPhase, DiscardPhase>;
}

/// Trait for actions during DiscardPhase.
pub(crate) trait DiscardActions where Self: HasGamePhase<DiscardPhase> + Sized {
    /// Discard a card for current player at given index in their hand.
    fn discard(&mut self, card_i: usize) -> Result<(), String>;

    /// Transition to the next state by going to the next active player
    /// where they can draw.
    /// 
    /// **NOTE**: Ensure a card is automatically discarded if it hasn't been when this is called,
    /// as the transition is infallible.
    fn to_next_player(self) -> TransitionResult<Self, DiscardPhase, DrawPhase>;
}

/// Trait for actions during RoundEndPhase.
pub(crate) trait RoundEndActions where Self: HasGamePhase<RoundEndPhase> + Sized {
    /// Calculate the round's score.
    fn calculate_score(&mut self) -> Result<(), String>;

    /// Start a new round by resetting the game's state, dealing out cards,
    /// and setting `DrawPhase` for the first player to draw.
    /// 
    /// **NOTE**: Ensure that score is automatically calculated if it hasn't been when this is called,
    /// as the transition is infallible.
    fn to_next_round(self) -> TransitionResult<Self, RoundEndPhase, DrawPhase>;
}

/// Trait for actions during GameEndPhase.
pub(crate) trait GameEndActions where Self: HasGamePhase<GameEndPhase> {
    // TODO: what makes sense here?
}

/// Trait for actions during any playable phase.
pub(crate) trait PlayableActions<P: PlayablePhase + GamePhase> where Self: HasGamePhase<P> {
    /// Add a player to the game.
    /// If an index is given, add them at that index in `players`;
    /// Else, add them at the last position of `players`.
    /// 
    /// If the player was added in the middle of a round, add them as inactive.
    fn add_player(&mut self, player_id: usize, index: Option<usize>);

    /// Sets a player as having quit.
    fn quit_player(&mut self, player_i: usize) -> Result<(), String>;
}