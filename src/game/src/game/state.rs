/// Trait indicating a game, whose state is tracked by `P: GamePhase`.
trait Game<P: GamePhase> {}

/// Trait indicating a game phase.
trait GamePhase {}

/// Trait indicating a phase where the game can still be played.
trait PlayablePhase {}

// GamePhase options.
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


/// Trait for actions during DrawPhase.
trait DrawActions where Self: Game<DrawPhase> {
    /// Draw from the stock for the current player.
    fn draw_stock(&mut self) -> Result<(), String>;

    /// Draw from the discard pile for the current player.
    fn draw_discard_pile(&mut self) -> Result<(), String>;
}

/// Trait for actions during PlayPhase.
trait PlayActions where Self: Game<PlayPhase> {
    /// Form a meld from a Vec of indices,
    /// referring to cards in the current player's hand.
    fn form_meld(&mut self, card_indices: Vec<usize>) -> Result<(), String>;

    /// Layoff `card_i` card in the current player's hand,
    /// to `target_player_i` player's `target_meld_i` meld.
    fn layoff_card(&mut self, card_i: usize, target_player_i: usize, target_meld_i: usize) -> Result<(), String>;
}

/// Trait for actions during DiscardPhase.
trait DiscardActions where Self: Game<DiscardPhase> {
    /// Discard a card for current player at given index in their hand.
    fn discard(&mut self, card_i: usize) -> Result<(), String>;
}

/// Trait for actions during RoundEndPhase.
trait RoundEndActions where Self: Game<RoundEndPhase> {
    /// Calculate the round's score.
    fn calculate_score(&mut self) -> Result<(), String>;
}

/// Trait for actions during any playable phase.
trait PlayableActions<P: PlayablePhase> where Self: Game<P> {
    /// Add a player to the game.
    /// If an index is given, add them at that index in `players`;
    /// Else, add them at the last position of `players`.
    /// 
    /// If the player was added in the middle of a round, add them as inactive.
    fn add_player(&mut self, player_id: usize, index: Option<usize>);

    /// Sets a player as having quit.
    fn quit_player(&mut self, player_i: usize) -> Result<(), String>;
}


/// The result of a game phase transition:
/// - Next: The logical next phase (ie Draw -> Play, Play -> Discard).
/// - End: The round has ended (due to some condition).
enum NextPhase<G: Game<P>, P: GamePhase> {
    Next(G),
    End
}

/// Trait for transitioning from one phase to another.
/// 
/// As it is infallible, there should be some default behaviour if the game 
/// currently cannot transition logically.
/// 
/// For example, if `next()` is called during DrawPhase, but the player hasn't drawn yet,
/// a card should automatically be drawn so the transition can still occur.
trait PhaseTransition<G: Game<P>, P: GamePhase> where Self: Game<P> {
    fn next(self) -> NextPhase<G, P>;
}

impl<G: Game<P>, P: GamePhase, T: Game<DrawPhase>> PhaseTransition<G, P> for T {
    fn next(self) -> NextPhase<G, PlayPhase> {
        todo!()
    }
}

pub struct Foo<P: GamePhase>(P);

impl<P: GamePhase> Game<P> for Foo<P> {
}