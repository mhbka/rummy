use axum::middleware::Next;

/// Trait indicating a game phase.
trait GamePhase {}

/// Trait indicating a phase where the game can still be played.
trait PlayablePhase {}

/// GamePhase options.
struct DrawPhase {
    pub(super) has_drawn: bool
}
struct PlayPhase {
    pub(super) move_count: usize
}
struct DiscardPhase {
    pub(super) has_discarded: usize
}
struct RoundEndPhase {
    pub(super) has_scored_round: bool
}
struct GameEndPhase {
    // no state needed, game has ended
}

// Mark these structs as GamePhases.
impl GamePhase for DrawPhase {}
impl GamePhase for PlayPhase {}
impl GamePhase for DiscardPhase {}
impl GamePhase for RoundEndPhase {}
impl GamePhase for GameEndPhase {}

impl PlayablePhase for DrawPhase {}
impl PlayablePhase for PlayPhase {}
impl PlayablePhase for DiscardPhase {}
impl PlayablePhase for RoundEndPhase {}


/// Enum that represents the result of a game phase transition:
/// - Next: The logical next phase (ie Draw -> Play).
/// - End: The round has ended (due to some condition).
pub enum NextPhase<P: GamePhase> {
    Next(P),
    End(G<RoundEndPhase>)
}

/// Trait for transitioning from one phase to another.
/// 
/// As it is infallible, there should be some default behaviour if the game 
/// currently cannot transition logically.
/// 
/// For example, if `next()` is called during DrawPhase, but the player hasn't drawn yet,
/// a stock card will automatically be drawn so the transition can still occur.
pub trait PhaseTransition<P: GamePhase> {
    fn next(self) -> NextPhase<P>;
}

/// Trait for actions during DrawPhase.
pub trait DrawActions {
    fn draw_stock(&mut self) -> Result<(), String>;
    fn draw_discard_pile(&mut self) -> Result<(), String>;
}

/// Trait for actions during PlayPhase.
pub trait PlayActions {
    /// Form a meld from a Vec of indices,
    /// referring to cards in the current player's hand.
    fn form_meld(&mut self, card_indices: Vec<usize>) -> Result<(), String>;

    /// Layoff a chosen card in the current player's hand,
    /// to a chosen player's chosen meld.
    fn layoff_card(&mut self, card_i: usize, target_player_i: usize, target_meld_i: usize) -> Result<(), String>;
}

/// Trait for actions during DiscardPhase.
pub trait DiscardActions {
    /// Discard a card for current player at given index in their hand.
    fn discard(&mut self, card_i: usize) -> Result<(), String>;
}

/// Trait for actions during RoundEndPhase.
pub trait RoundEndActions {
    type EndedGame;

    /// Calculate the round's score.
    fn calculate_score(&mut self) -> Result<(), String>;

    /// End the game.
    fn end_game(self) -> Self::EndedGame;
}

/// Trait for actions during any playable phase.
pub trait PlayableActions {
    /// Add a player to the game.
    /// If an index is given, add them at that index in `players`;
    /// Else, add them at the last position of `players`.
    /// 
    /// If the player was added in the middle of a round, add them as inactive.
    fn add_player(&mut self, player_id: usize, index: Option<usize>);

    /// Sets a player as having quit.
    fn quit_player(&mut self, player_i: usize);
}

pub struct Game<P: GamePhase> {
    phase: P
}

impl PhaseTransition<P: GamePhase> for Game<DrawActions> {
    typ

    fn next(self) -> NextPhase<P> {
        
    }
}

