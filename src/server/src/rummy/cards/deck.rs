use super::card::Card;
use super::suit_rank::Rank;

pub struct DeckConfig {
    pub high_rank: Rank,
    pub wildcard_rank: Option<Rank>
}

pub struct Deck {
    pub(crate) config: DeckConfig,
    pub(crate) cards: Vec<Card>
}