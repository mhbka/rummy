/// The discrete phases of a Rummy game.
/// Each phase allows/forbids certain behaviours within `Game`.
pub enum GamePhase {
    /// The current player can draw from the deck or discard pile.
    PlayerDraw,

    /// The current player can play valid moves.
    PlayerPlays,

    /// The current player can discard a card.
    PlayerDiscard,

    /// The current round has ended.
    RoundEnd,
    
    /// The game has ended.
    GameEnd
}

/// The state of a `Game`.
pub struct GameState {
    phase: GamePhase,
    round_number: isize,
    player_index: usize,
}

impl GameState {
    pub(super) fn new() -> Self {
        GameState {
            phase: GamePhase::RoundEnd,
            round_number: -1,
            player_index: 0
        }
    }
}