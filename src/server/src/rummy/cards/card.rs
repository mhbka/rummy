use serde::{Serialize, Deserialize};
use super::{deck::Deck, suit_rank::{Rank, Suit}};
use std::{rc::Rc, cmp::Ordering};

/// A card.
/// 
/// Always tied to a `Deck`.
#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
    pub(crate) rank: Rank,
    pub(crate) suit: Suit,

    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) deck: Rc<Deck> 
    // TODO: make this Option so we can default it to None for serde
    // TODO: then figure out how to Rc to the deck upon deserializing
}

impl Card {
    /// Creates a new card.
    ///  
    /// Typically this is done inside a `Deck` instantiation,
    /// as the card depends on the deck's configuration for comparisons.
    pub(super) fn new(deck: Rc<Deck>, rank: Rank, suit: Suit) -> Self {
        Card {
            rank,
            suit,
            deck
        }
    }

    /// Gets the card's rank and suit.
    pub fn data(&self) -> (Rank, Suit) {
        (self.rank, self.suit)
    }
}


/// Basic equality impls.
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.rank == other.rank
            && self.suit == other.suit
    }
}

impl Eq for Card {}


/// Compares cards by rank, then suit.
/// 
/// For rank, we offset by the high rank provided in the deck's config (if there is one).
/// Thus, the deck can use any rank as high rank,
/// and ordering will count down from there.
///
/// For example, if high rank is 2,
/// then 2 > Ace > King ... 4 > 3.
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank == other.rank {
            self.suit.cmp(&other.suit)
        }
        else {
            let max_rank = Rank::King as u8;
            let highest_rank = if self.deck.config.high_rank.is_none() { 
                max_rank 
            } else {
                self.deck.config.high_rank.unwrap() as u8
            };
            let rank_offset = max_rank - highest_rank;

            let self_rank = (self.rank as u8 + rank_offset) % (max_rank+1);
            let other_rank = (other.rank as u8 + rank_offset) % (max_rank+1);
            self_rank.cmp(&other_rank)
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}