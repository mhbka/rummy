use std::rc::Rc;

use super::card::Card;
use super::suit_rank::{Rank, Suit};
use strum::IntoEnumIterator;
use rand::{
    SeedableRng,
    seq::SliceRandom,
    rngs::StdRng
};

/// Configurable parameters for a deck:
/// - `shuffle_seed`: Optional seed for deterministically shuffling the deck
/// - `pack_count`: Number of card packs to include in the deck
/// - `use_joker`: Whether to add Jokers and use them as wildcard (2 per pack)
/// - `high_rank`: Whether to override the highest rank (default being King)
/// - `wildcard_rank`: Whether to have a wildcard rank (mutually exclusive with `use_joker`)
#[derive(Default, Debug)]
pub struct DeckConfig {
    pub shuffle_seed: Option<u64>,
    pub pack_count: usize,
    pub use_joker: bool,
    pub high_rank: Option<Rank>,
    pub wildcard_rank: Option<Rank>
}

// TODO: verify cards belong to the deck before adding to discard pile

/// The deck, consisting of the:
/// - **stock**, face-down cards that can be drawn at the start of each turn
/// - **discard pile**, discarded cards, which can also be drawn
#[derive(Debug)]
pub struct Deck {
    config: Rc<DeckConfig>,
    stock: Vec<Card>,
    discard_pile: Vec<Card>
}

impl Deck {
    /// Creates a new deck following settings in `config` and shuffles it.
    /// 
    /// **Note**: 
    /// - If `pack_count` < 1, it will be set to 1.
    /// - If `use_joker` is true while `wildcard_rank` is not `None`, `use_joker` will default to `false`.
    pub(crate) fn new(mut config: DeckConfig) -> Self {
        if config.pack_count < 1 {
            config.pack_count = 1;
        }
        if config.wildcard_rank.is_some() && config.use_joker {
            config.use_joker = false;
        }

        let config = Rc::new(config);

        let mut deck = Deck {
            config: config.clone(),
            stock: Vec::new(),
            discard_pile: Vec::new()
        };

        Deck::generate_cards(&mut deck.stock, &config);
        Deck::shuffle_cards(&mut deck.stock, &config);

        deck
    }

    /// Reset the cards by creating a new deck and shuffling it.
    /// 
    /// **NOTE**: This refers to the current `DeckConfig`; if it has changed,
    /// the cards generated will be different from what was initially generated.
    pub(crate) fn reset(&mut self) {
        self.stock.clear();
        self.discard_pile.clear();
        Deck::generate_cards(&mut self.stock, &self.config);
        Deck::shuffle_cards(&mut self.stock, &self.config);
    }

    /// Draw `amount` cards from the deck stock.
    /// 
    /// If `amount` is greater than the stock size, `Err` is returned.
    /// 
    /// To replenish the stock, one can call `shuffle_discarded` or `turnover_discarded`.
    pub(crate) fn draw(&mut self, amount: usize) -> Result<Vec<Card>, String> {
        if amount > self.stock.len() {
            return Err(format!("Draw amount ({amount}) greater than stock size ({})", self.stock.len()));
        }

        let cards = self.stock.split_off(self.stock.len() - amount);
        Ok(cards)
    }

    /// Draw a specific card from the deck stock.
    /// 
    /// If the card doesn't exist in the stock, return `Err`.
    /// 
    /// If the deck is empty after drawing, shuffle the discarded cards back into it.
    pub(crate) fn draw_specific(&mut self, rank: Rank, suit: Suit) -> Result<Card, String> {
        for i in 0..self.stock.len() {
            let card = &self.stock[i];
            if card.rank == rank && card.suit == suit {
                return Ok(self.stock.remove(i));
            }
        }

        Err(format!("No card ({suit:?}, {rank:?}) in the stock"))
    }

    /// See the top card of the discard pile, if there is one.
    pub(crate) fn peek_discard_pile(&self) -> Option<(Rank, Suit)> {
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
    pub(crate) fn draw_discard_pile(&mut self, amount: Option<usize>) -> Result<Vec<Card>, String> {
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
    pub(crate) fn add_to_discard_pile(&mut self, cards: &mut Vec<Card>) {
        self.discard_pile.append(cards);
    }

    /// Reset the stock by moving the discard pile into it and shuffling.
    pub(crate) fn shuffle_discarded(&mut self) {
        self.stock.append(&mut self.discard_pile);
        self.stock.shuffle(&mut rand::thread_rng());
    }

    /// Reset the stock by moving the discard pile into it and turning it over.
    pub(crate) fn turnover_discarded(&mut self) {
        self.stock.append(&mut self.discard_pile);
        self.stock.reverse();
    }

    /// Get a reference to the deck configuration.
    pub(crate) fn config(&self) -> &DeckConfig {
        &self.config
    }

    /// Get a reference to the deck stock.
    pub(crate) fn stock(&self) -> &Vec<Card> {
        &self.stock
    }

    /// Get a reference to the deck discard pile.
    pub(crate) fn discard_pile(&self) -> &Vec<Card> {
        &self.discard_pile
    }

    /// Generating cards into a `stock` based on `config`.
    fn generate_cards(stock: &mut Vec<Card>, config: &Rc<DeckConfig>) {
        for _ in 0..config.pack_count {
            for suit in Suit::iter() {
                if suit == Suit::Joker { continue; }
                for rank in Rank::iter() {
                    if rank == Rank::Joker { continue; }
                    stock.push(Card { rank, suit, deck_config: config.clone() });
                }
            }

            if config.use_joker {
                stock.push(Card { 
                    rank: Rank::Joker, 
                    suit: Suit::Joker, 
                    deck_config: config.clone() 
                });
            }
        }
    }

    /// Shuffles cards in a `stock` based on `config`.
    fn shuffle_cards(stock: &mut Vec<Card>, config: &Rc<DeckConfig>) {
        match config.shuffle_seed {
            Some(seed) => stock.shuffle(&mut StdRng::seed_from_u64(seed)),
            None => stock.shuffle(&mut rand::thread_rng())
        }
    }
}