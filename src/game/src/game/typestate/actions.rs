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


/// A supertrait encompassing all actions.
pub(crate) trait GameActions:
    DrawActions
    + PlayActions 
    + DiscardActions
    + RoundEndActions
    + GameEndActions
{}


/// Trait for actions during DrawPhase.
pub(crate) trait DrawActions where Self: HasGamePhase<DrawPhase> {
    /// Draw from the stock for the current player.
    fn draw_stock(&mut self) -> Result<(), String>;

    /// Draw from the discard pile for the current player.
    fn draw_discard_pile(&mut self) -> Result<(), String>;
}

/// Trait for actions during PlayPhase.
pub(crate) trait PlayActions where Self: HasGamePhase<PlayPhase> {
    /// Form a meld from a Vec of indices,
    /// referring to cards in the current player's hand.
    fn form_meld(&mut self, card_indices: Vec<usize>) -> Result<(), String>;

    /// Layoff `card_i` card in the current player's hand,
    /// to `target_player_i` player's `target_meld_i` meld.
    fn layoff_card(&mut self, card_i: usize, target_player_i: usize, target_meld_i: usize) -> Result<(), String>;
}

/// Trait for actions during DiscardPhase.
pub(crate) trait DiscardActions where Self: HasGamePhase<DiscardPhase> {
    /// Discard a card for current player at given index in their hand.
    fn discard(&mut self, card_i: usize) -> Result<(), String>;
}

/// Trait for actions during RoundEndPhase.
pub(crate) trait RoundEndActions where Self: HasGamePhase<RoundEndPhase> {
    /// Calculate the round's score.
    fn calculate_score(&mut self) -> Result<(), String>;
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