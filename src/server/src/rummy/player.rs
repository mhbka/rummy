use super::cards::{card::Card, meld::Meld};

pub struct Player {
    pub(super) id: usize,
    pub(super) cards: Vec<Card>,
    pub(super) melds: Vec<Meld>,
    pub(super) active: bool
}

impl Player {
    pub(super) fn new(id: usize) -> Self {
        Player {
            id,
            cards: Vec::new(),
            melds: Vec::new(),
            active: true
        }
    }
}