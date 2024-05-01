use super::cards::{card::Card, meld::Meld};

/// A Rummy player.
pub(crate) struct Player {
    pub(crate) id: usize,
    pub(crate) cards: Vec<Card>,
    pub(crate) melds: Vec<Meld>,
    pub(crate) active: bool
}

impl Player {
    /// Creates a new player.
    pub(crate) fn new(id: usize) -> Self {
        Player {
            id,
            cards: Vec::new(),
            melds: Vec::new(),
            active: true
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