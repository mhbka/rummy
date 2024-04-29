use super::card::Card;
use super::suit_rank::{Rank, Suit};
use rand::seq::SliceRandom;

pub struct DeckConfig {
    pub high_rank: Rank,
    pub wildcard_rank: Option<Rank>
}

// TODO: move discard pile + related fns into Game
pub struct Deck {
    pub(crate) config: DeckConfig,
    pub(crate) cards: Vec<Card>,
    pub(crate) discard_cards: Vec<Card>
}

impl Deck {
    pub(super) fn new() -> Self {
        todo!()
    }

    /// Draw a single card from the deck.
    /// 
    /// If the deck is empty after drawing, shuffle the discarded cards back into it.
    pub fn draw(&mut self) -> Card {
        let card = self.cards.pop().unwrap();
        if self.cards.len() == 0 {
            self.reset_deck();
        };
        card
    }

    /// See the top card of the discard pile, if there is one.
    pub fn peek_discard_pile(&self) -> Option<(Rank, Suit)> {
        self.discard_cards
            .last()
            .map(|card| card.data())
    }

    /// Attempt to draw a chosen amount of cards from the discard pile.
    /// 
    /// If the amount is greater than discard pile's size, or the discard pile is empty,
    /// return an `Err`.
    /// 
    /// If `None` amount is specified, attempt to draw the entire discard pile.
    pub fn draw_discard_pile(&mut self, amount: Option<usize>) -> Result<Vec<Card>, ()> {
        let discard_size = self.discard_cards.len();
        if discard_size == 0 {
            return Err(());
        }
        else if let Some(a) = amount {
            if a > discard_size {
                return Err(());
            }
            return Ok(
                self.discard_cards.split_off(discard_size - a)
            );
        }
        return Ok(
            self.discard_cards.split_off(0)
        );
    }

    /// Moves cards from `cards` into the discard pile, leaving it empty.
    pub fn add_to_discard_pile(&mut self, cards: &mut Vec<Card>) {
        self.discard_cards.append(cards);
    }

    /// Reset the deck by moving the discard pile into it and shuffling.
    /// 
    /// Typically called when deck is emptied during gameplay,
    /// or when starting a new round (and all player cards have been discarded).
    fn reset_deck(&mut self) {
        self.cards.append(&mut self.discard_cards);
        self.cards.shuffle(&mut rand::thread_rng());
    }
}

impl Default for Deck {
    fn default() -> Self {
        todo!()
    }
}