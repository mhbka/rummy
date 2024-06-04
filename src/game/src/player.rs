use super::cards::{card::Card, meld::Meld};

/// A Rummy player.
pub(crate) struct Player {
    pub(crate) id: usize,
    pub(crate) cards: Vec<Card>,
    pub(crate) melds: Vec<Meld>,
    pub(crate) active: bool,
    pub(crate) joined_in_round: usize
}

impl Player {
    /// Creates a new player.
    pub(crate) fn new(id: usize, active: bool, joined_in_round: usize) -> Self {
        Player {
            id,
            cards: Vec::new(),
            melds: Vec::new(),
            active,
            joined_in_round
        }
    }

    /// Resets a player's state.
    /// 
    /// **Note**: This destroys their hand/meld cards, 
    /// so a new deck should be created.
    pub(crate) fn reset(&mut self) {
        self.cards.clear();
        self.melds.clear();
    }
}