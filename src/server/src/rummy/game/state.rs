/// Sealed trait indicating a game phase.
trait GamePhase {}

/// GamePhase options.
struct DrawPhase {}
struct PlayPhase {}
struct DiscardPhase {}
struct RoundEndPhase {}
struct GameEndPhase {}

impl GamePhase for DrawPhase {}
impl GamePhase for PlayPhase {}
impl GamePhase for DiscardPhase {}
impl GamePhase for RoundEndPhase {}
impl GamePhase for GameEndPhase {}


/// Trait indicating a game.
trait Gameable<P: GamePhase> {}


/// The internal state of a `Game`.
pub struct GameState<P: GamePhase> {
    round_number: isize,
    player_index: usize,
    phase: P
}

impl GameState<RoundEndPhase> {
    pub(super) fn new() -> Self {
        GameState {
            round_number: 0,
            player_index: 0
        }
    }
}

pub struct Game<P: GamePhase> {
    pub(super) deck: Deck,
    pub(super) players: Vec<Player>,
    state: GameState<P>
}

impl Game<DrawPhase> {
    fn draw(&mut self) { /* draw card */ }
    fn next(mut self) -> Game<PlayPhase> { /* change internal state */ }
}

impl Game<PlayPhase> {
    fn form_meld(&mut self) { /* form meld from cards */ }
    fn next(mut self) -> Game<PlayPhase> { /* change internal state */ }
}

/* same idea for other */



