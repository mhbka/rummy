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
    fn try_add_card(&mut self, card: Card) -> Result<(), Card>;
}


/// A Rummy meld set.
pub struct Set {
    cards: Vec<Card>,
    pub(crate) set_rank: Rank
}

impl Meldable for Set {
    fn new(mut cards: Vec<Card>) -> Result<Self, Vec<Card>> {
        // assuming every card is tied to the same `deck_config`
        match cards[0].deck_config.wildcard_rank { 
            // every card has same rank, or is a wildcard.
            Some(wildcard_rank) => {
                let mut set_rank: Option<Rank> = None;
                if cards
                    .iter()
                    .all(|card| {
                        if card.rank == wildcard_rank { return true; }
                        else {
                            match set_rank {
                                Some(rank) => return card.rank == rank,
                                None => {
                                    set_rank = Some(card.rank);
                                    return true;
                                }
                            }
                        }
                    }) {
                    if let Some(set_rank) = set_rank {
                        return Ok(Set{set_rank, cards});
                    }
                    else { // every card is a wildcard, which is not a valid set.
                        return Err(cards);
                    }
                }
                else {
                    return Err(cards);
                }
                
            },
           
            None => { // every card has same rank.
                if cards
                    .iter()
                    .all(|card| card.rank == cards[0].rank) {
                    return Ok(Set{set_rank: cards[0].rank, cards});
                }   
                else {
                    return Err(cards);
                }
            }
        }
    }

    fn try_add_card(&mut self, card: Card) -> Result<(), Card> {
        if card.rank != self.set_rank { 
            return Err(card); 
        }
        else if let Some(wildcard_rank) = card.deck_config.wildcard_rank {
            if card.rank != wildcard_rank {
                return Err(card);
            }
        }
        self.cards.push(card);
        Ok(())
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

        cards.sort();

        // TODO: do I just assume that every card is tied to the same deck?
        let deck_config = cards[0].deck_config.clone();

        let mut wildcards = match deck_config.wildcard_rank {
            Some(wildcard_rank) => {
                cards.iter().filter(|&card| card.rank == wildcard_rank).collect()
            },
            None => {
                Vec::new()
            }
        };
        

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
                        let wildcard = wildcards.pop();
                        let wildcard = wildcard.unwrap();
                        cards.insert(i, wildcard.clone());
                        continue;
                    }
                } 
                return Err(backup_cards);
            }
        }

        Ok(Run { cards })
    }

    fn try_add_card(&mut self, card: Card) -> Result<(), Card> {
        todo!();
    }
}