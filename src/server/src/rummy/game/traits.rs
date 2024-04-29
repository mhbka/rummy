/// A Rummy variant must minimally implement these traits.
pub trait Gameable: GameInit + GameActions + GameAdmin {}

// TODO: eventually convert String to custom errors

pub trait GameInit {
    /// The configuration for the game.
    type Config;

    /// Creates a new game with the given player IDs.
    fn new(player_ids: Vec<usize>, config: Self::Config) -> Result<Self, String> 
    where Self: Sized;

    /// Deals active players the configured amount of cards for this game variant,
    /// only during `GamePhase::RoundEnd`.
    fn init_round(&mut self) -> Result<(), String>;
}

pub trait GameActions {
    /// Draws a card from the deck for the current player,
    /// only during `GamePhase::PlayerDraw`.
    fn draw_deck(&mut self) -> Result<(), String>;

    /// Draws the configured amount of cards from the discard pile for the current player,
    /// only during `GamePhase::PlayerDraw`.
    fn draw_discard_pile(&mut self) -> Result<(), String>;

    /// Attempts to form a meld for the current player using a `Vec` of card indices,
    /// only during `GamePhase::PlayerPlays`.
    fn form_meld(&mut self, indices: Vec<usize>) -> Result<(), String>;

    /// Attempts to layoff a chosen card of the current player to a chosen meld of a chosen player,
    /// only during `GamePhase::PlayerPlays`.
    fn layoff_card(
        &mut self, 
        card_index: usize, 
        target_player_index: usize, 
        target_meld_index: usize) -> Result<(), String>;

    /// Attempts to layoff a chosen card of the current player to a chosen meld of a chosen player,
    /// only during `GamePhase::PlayerDiscard`.
    fn discard_card(&mut self, card_index: usize) -> Result<(), String>;
}

pub trait GameAdmin {
    /// Adds a new player at the given index of players.
    /// If index is `None`, the player is placed at last index.
    /// 
    /// If `GamePhase::RoundEnd`, the player is added as active;
    /// otherwise, they are inactive until the next round starts.
    /// 
    /// If `GamePhase::GameEnd`, this returns `Err`.
    fn player_join(&mut self, player_id: usize, index: Option<usize>) -> Result<(), String>;

    /// Quits a player in the middle of a game.
    /// This sets them as inactive, and still computes their score for the round at the end.
    /// 
    /// If `GamePhase::GameEnd`, this returns `Err`.
    fn player_quit(&mut self, index: usize) -> Result<(), String>;

    /// Scores the players of a game,
    /// only during `GamePhase::GameEnd`.
    fn calculate_score(&mut self) -> Result<(), String>;
}