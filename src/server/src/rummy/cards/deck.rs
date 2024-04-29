use std::rc::Rc;

use super::card::Card;
use super::suit_rank::{Rank, Suit};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

/// Configurable parameters for a deck:
/// - `pack_count`: Number of card packs to include in the deck
/// - `use_joker`: Whether to add Jokers and use them as wildcard (2 per pack)
/// - `high_rank`: Whether to override the highest rank (default being King)
/// - `wildcard_rank`: Whether to have a wildcard rank (mutually exclusive with `use_joker`)
pub struct DeckConfig {
    pub pack_count: usize,
    pub use_joker: bool,
    pub high_rank: Option<Rank>,
    pub wildcard_rank: Option<Rank>
}

// TODO: verify cards belong to the deck before adding to discard pile
// TODO: some variants may allow see multiple discarded cards; possible functionality for that

/// The deck, consisting of the:
/// - **stock**, face-down cards that can be drawn at the start of each turn
/// - **discard pile**, discarded cards, which can also be drawn
pub struct Deck {
    pub(super) config: DeckConfig,
    pub(super) stock: Vec<Card>,
    pub(super) discard_pile: Vec<Card>
}

impl Deck {
    /// Creates a new deck following settings in `config`.
    /// 
    /// **Note**: Returns `Err` if `pack_count` < 1, or `use_joker` is true while `wildcard_rank` isn't `None`.
    /// TODO: why can't I make this pub(super) without angering basic.rs?
    pub(crate) fn new(config: DeckConfig) -> Result<Self, String> {
        if config.pack_count < 1 {
            return Err("Pack count < 1 while instantiating a Deck".to_owned());
        }
        if config.wildcard_rank.is_some() && config.use_joker {
            return Err("Cannot use Joker and specify a wildcard in a Deck".to_owned());
        }

        let mut deck = Deck {
            config,
            stock: Vec::new(),
            discard_pile: Vec::new()
        };

        for i in 0..config.pack_count {
            for suit in Suit::iter() {
                if suit == Suit::Joker { continue; }
                for rank in Rank::iter() {
                    if rank == Rank::Joker { continue; }
                    deck.stock.push(Card { rank, suit, deck: Rc::new(deck) });
                }
            }
            if config.use_joker {
                deck.stock.push(Card { rank: Rank::Joker, suit: Suit::Joker, deck: Rc::new(deck) });
            }
        }

        Ok(deck)
    }

    /// Draw a chosen amount of cards (usually 1) from the deck stock.
    /// 
    /// If the amount is greater than the stock's size, return `Err`.
    /// 
    /// If the deck is empty after drawing, shuffle the discarded cards back into it.
    pub(super) fn draw(&mut self, amount: usize) -> Result<Card, String> {
        if amount > self.stock.len() {
            return Err(format!("Draw amount ({amount}) greater than stock size ({})", self.stock.len()));
        }

        let cards = self.stock.split_off(self.stock.len() - amount);
        if self.stock.len() == 0 {
            self.reset_deck();
        };
        
        Ok(cards)
    }

    /// See the top card of the discard pile, if there is one.
    pub(super) fn peek_discard_pile(&self) -> Option<(Rank, Suit)> {
        self.discard_pile
            .last()
            .map(|card| card.data())
    }

    /// Attempt to draw a chosen amount of cards from the discard pile.
    /// 
    /// If the amount is greater than discard pile's size, or the discard pile is empty,
    /// return `Err`.
    /// 
    /// If `None` amount is specified, attempt to draw the entire discard pile.
    pub(super) fn draw_discard_pile(&mut self, amount: Option<usize>) -> Result<Vec<Card>, String> {
        let discard_size = self.discard_pile.len();
        if discard_size == 0 {
            return Err(format!("Can't draw from empty discard pile"));
        }
        else if let Some(a) = amount {
            if a > discard_size {
                return Err(format!("Draw amount ({a}) greater than discard pile size ({discard_size})"));
            }
            return Ok(
                self.discard_pile.split_off(discard_size - a)
            );
        }
        return Ok(
            self.discard_pile.split_off(0)
        );
    }

    /// Moves cards from `cards` into the discard pile, leaving it empty.
    pub(super) fn add_to_discard_pile(&mut self, cards: &mut Vec<Card>) {
        self.discard_pile.append(cards);
    }

    /// Reset the stock by moving the discard pile into it and shuffling.
    /// 
    /// Typically called when stock is emptied during gameplay,
    /// or when starting a new round (and all player cards have been discarded).
    pub(super) fn reset_deck(&mut self) {
        self.stock.append(&mut self.discard_pile);
        self.stock.shuffle(&mut rand::thread_rng());
    }
}

impl Default for Deck {
    fn default() -> Self {
        todo!()
    }
}