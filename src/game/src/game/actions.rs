use super::phases::{
    GamePhase,
    PlayablePhase,
    DrawPhase,
    PlayPhase,
    DiscardPhase,
    RoundEndPhase,
    GameEndPhase,
};


/// Trait for actions during DrawPhase.
pub(crate) trait DrawActions {
    // `Self` in `PlayPhase` and `RoundEndPhase`.
    type SelfInPlayPhase: PlayActions;
    type SelfInRoundEndPhase: RoundEndActions;
 
    /// Draw from the stock for the current player.
    fn draw_stock(&mut self) -> Result<(), String>;

    /// Draw from the discard pile for the current player.
    /// 
    /// `amount` is the number of cards to draw, where `None` means draw the entire discard pile.
    /// 
    /// This is provided as some variants of Rummy (in particular [Basic Rummy](https://en.wikipedia.org/wiki/Rummy))
    /// may allow the player to choose how many discard cards to draw.
    /// 
    /// If a variant doesn't allow this, the implementation can just ignore `amount` and always use a default value.
    fn draw_discard_pile(&mut self, amount: Option<usize>) -> Result<(), String>;

    /// Transition to next state where the current player can make plays.
    /// 
    /// **NOTE**: Ensure any required actions are taken by the time/during this function call,
    /// as it is infallible.
    fn to_play(self) -> Result<Self::SelfInPlayPhase, Self::SelfInRoundEndPhase>;
}

/// Trait for actions during PlayPhase.
pub(crate) trait PlayActions {
    // `Self` in `DiscardPhase` and `RoundEndPhase`.
    type SelfInDiscardPhase: DiscardActions;
    type SelfInRoundEndPhase: RoundEndActions;

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
    fn to_discard(self) -> Result<Self::SelfInDiscardPhase, Self::SelfInRoundEndPhase>;
}

/// Trait for actions during DiscardPhase.
pub(crate) trait DiscardActions {
    // `Self` in `PlayPhase` and `RoundEndPhase`.
    type SelfInDrawPhase: DrawActions;
    type SelfInRoundEndPhase: RoundEndActions;

    /// Discard a card for current player at given index in their hand.
    fn discard(&mut self, card_i: usize) -> Result<(), String>;

    /// Transition to the next state by going to the next active player
    /// where they can draw.
    /// 
    /// **NOTE**: Ensure a card is automatically discarded if it hasn't been when this is called,
    /// as the transition is infallible.
    fn to_next_player(self) -> Result<Self::SelfInDrawPhase, Self::SelfInRoundEndPhase>;
}

/// Trait for actions during RoundEndPhase.
pub(crate) trait RoundEndActions {
    // `Self` in `PlayPhase` and `RoundEndPhase`.
    type SelfInPlayPhase: PlayActions;

    /// Calculate the round's score.
    fn calculate_score(&mut self) -> Result<(), String>;

    /// Start a new round by resetting the game's state, dealing out cards,
    /// and setting `DrawPhase` for the first player to draw.
    /// 
    /// **NOTE**: Ensure that score is automatically calculated if it hasn't been when this is called,
    /// as the transition is infallible.
    fn to_next_round(self) -> Self::SelfInPlayPhase;
}

/// Trait for actions during GameEndPhase.
pub(crate) trait GameEndActions {
    // TODO: what makes sense here?
}

/// Trait for actions during any playable phase.
pub(crate) trait PlayableActions: Sized {
    type SelfInRoundEndPhase: RoundEndActions;
    type SelfInDrawPhase: DrawActions;

    /// Add a player to the game.
    /// If an index is given, add them at that index in `players`;
    /// Else, add them at the last position of `players`.
    /// 
    /// If the player was added while a round is ongoing, add them as inactive.
    fn add_player(&mut self, player_id: usize, index: Option<usize>);

    /// Sets a (non-current) player as having quit.
    /// If only 1 active player is left, ends the round.
    /// 
    /// Nothing happens if `player_i` is the current player.
    /// To quit the current player, use `quit_current_player` instead.
    fn quit_player(self, player_i: usize) -> Result<Self, Self::SelfInRoundEndPhase>;

    /// Sets the current player as having quit, advancing to the next player
    /// and going to `DrawPhase`.
    fn quit_current_player(self) -> Self::SelfInDrawPhase;
}