use super::{card::Card, suit_rank::Rank};

/// A Rummy meld.
/// 
/// There are 2 types: a **set** (>=3 cards of same rank),
/// and **run** (>=3 sequential cards of same suit).
pub enum Meld {
    Set(Set),
    Run(Run)
}

pub trait Meldable {
    /// Attempt to create a new meld out of a Vec of `Card`s.
    /// 
    /// If valid, `Ok` is returned.
    /// Else, `Error` is returned along with the cards.
    /// 
    /// **NOTE**: in the `Error` case, the caller must ensure the cards are moved somewhere concrete,
    /// like a deck/player hand/discard pile.
    fn new(cards: Vec<Card>) -> Result<Self, Vec<Card>> where Self: Sized;

    /// Attempt to add a `Card` to the set.
    /// 
    /// If the new card fits into the meld, it is moved into the meld and `Ok` is returned.
    /// Else, `Error` is returned along with the card.
    /// 
    /// **NOTE**: in the `Error` case, the caller must ensure the card is moved somewhere concrete,
    /// like a deck/player hand/discard pile.
    fn try_add_card(index: usize, card: Card) -> Result<(), Card>;
}


/// A Rummy meld set.
pub struct Set {
    pub(crate) cards: Vec<Card>
}

impl Meldable for Set {
    fn new(mut cards: Vec<Card>) -> Result<Self, Vec<Card>> {
        // TODO: do I just assume that every card is tied to the same deck?
        let deck_config = &cards[0].deck.config;
        let mut wildcard_count = 0;
        let mut set_rank: Option<Rank> = None;

        for card in &cards {
            if let Some(wildcard_rank) = deck_config.wildcard_rank { 
                if wildcard_rank == card.rank {
                    wildcard_count += 1;
                    continue; 
                }
            }
            
            if let None = set_rank { 
                set_rank = Some(card.rank);
            }
            if set_rank.unwrap() != card.rank {
                if wildcard_count > 0 { wildcard_count -= 1; }
                else { return Err(cards); }
            }
        }

        Ok(Set { cards })
    }

    fn try_add_card(index: usize, card: Card) -> Result<(), Card> {
        todo!();
    }
}


/// A Rummy meld run.
pub struct Run {
    pub(crate) cards: Vec<Card>
}

impl Meldable for Run {
    fn new(mut cards: Vec<Card>) -> Result<Self, Vec<Card>> {
        // TODO: any way to not do this?
        let backup_cards = cards.clone();

        // TODO: do I just assume that every card is tied to the same deck?
        let deck_config = &cards[0].deck.config;

        let mut wildcards = match deck_config.wildcard_rank {
            Some(wildcard_rank) => {
                cards.iter().filter(|&card| card.rank == wildcard_rank).collect()
            },
            None => {
                Vec::new()
            }
        };
        
        cards.sort();

        // Check that each card is same suit and +1 rank from previous card (or previous card is wildcard).
        // If not, try to insert a wildcard; if we have none, return Error with the backup cards.
        for i in 1..cards.len() {
            if cards[i-1].suit == cards[i].suit
            && cards[i-1].rank as u8 == cards[i+1].rank as u8 + 1 {
                continue;
            }
            else {
                if let Some(wildcard_rank) = deck_config.wildcard_rank {
                    if cards[i-1].rank == wildcard_rank {
                        continue;
                    }
                    else if wildcards.len() > 0 {
                        let &wildcard = wildcards.pop().unwrap();
                        cards.insert(i, wildcard);
                        continue;
                    }
                } 
                return Err(backup_cards);
            }
        }

        Ok(Run { cards })
    }

    fn try_add_card(index: usize, card: Card) -> Result<(), Card> {
        todo!();
    }
}