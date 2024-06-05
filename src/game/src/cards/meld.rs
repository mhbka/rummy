use super::{card::Card, suit_rank::Rank};


pub trait Meldable {
    /// Attempt to create a new meld out of a Vec of `Card`s.
    /// 
    /// If valid, the Vec is drained and `Ok` is returned.
    /// Else, `Err` is returned and `cards` is left untouched.
    fn new(meld_cards: &mut Vec<Card>) -> Result<Self, String> where Self: Sized;

    /// Attempt to add a `Card` to the set.
    /// 
    /// If the new card fits into the meld, it is moved into the meld and `Ok` is returned.
    /// 
    /// Else, `Error` is returned along with the card.
    fn layoff_card(&mut self, card: Card) -> Result<(), Card>;
}


/// A Rummy meld.
/// There are 2 types: 
/// - **Set**; >=3 cards of same rank
/// - **Run**; >=3 sequential cards of same suit
pub enum Meld {
    Set(Set),
    Run(Run)
}

impl Meldable for Meld {
    fn new(meld_cards: &mut Vec<Card>) -> Result<Self, String> where Self: Sized {
        if let Ok(set) = Set::new(meld_cards) {
            Ok(Meld::Set(set))
        }
        else if let Ok(run) = Run::new(meld_cards) {
            Ok(Meld::Run(run))
        }
        else {
            Err("Cards don't form a valid set or run.".to_owned())
        }
    }

    fn layoff_card(&mut self, card: Card) -> Result<(), Card> {
        match self {
            Meld::Set(set) => set.layoff_card(card),
            Meld::Run(run) => run.layoff_card(card)
        }
    }
}


/// A Rummy meld set.
pub struct Set {
    cards: Vec<Card>,
    pub(crate) set_rank: Rank
}

impl Meldable for Set {
    fn new(meld_cards: &mut Vec<Card>) -> Result<Self, String> {
        let cards = meld_cards.clone();

        // assuming every card is tied to the same `deck_config`
        match cards[0].deck_config.wildcard_rank { 
            // we check if every card has same rank, or the wildcard rank
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

                    // set_rank has been set, and every card has this rank or the wildcard rank.
                    if let Some(set_rank) = set_rank {
                        return Ok(Set{ set_rank, cards });
                    }
                    // every card is a wildcard, which is not a valid set.
                    else { 
                        return Err("A set cannot be formed out of only wildcards".to_owned());
                    }
                }
                else {
                    return Err("Cards do not form a valid set".to_owned());
                }
                
            },
            
            // we check if every card has same rank
            None => { 
                if cards
                    .iter()
                    .all(|card| card.rank == cards[0].rank) {
                        meld_cards.clear();
                        return Ok(
                            Set{ set_rank: cards[0].rank, cards }
                        );
                }   
                else {
                    return Err("Cards do not form a valid set".to_owned());
                }
            }
        }
    }

    fn layoff_card(&mut self, card: Card) -> Result<(), Card> {
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
    fn new(meld_cards: &mut Vec<Card>) -> Result<Self, String> {
        // TODO: do I just assume that every card is tied to the same deck?
        let deck_config = meld_cards[0].deck_config.clone();

        let (mut cards, mut wildcards) = match deck_config.wildcard_rank {
            Some(wildcard_rank) => meld_cards
                .iter()
                .partition(|&c| c.rank == wildcard_rank),
            None => (meld_cards.iter().collect(), Vec::new())
        };

        // Check that each card is same suit and +1 rank from previous card (or previous card is wildcard).
        // If not, try to insert a wildcard.
        // If we have none, return Err.
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
                        let wildcard = wildcards.pop().unwrap();
                        cards.insert(i, wildcard);
                        continue;
                    }
                } 
                return Err("Cards don't form a valid run".to_owned());
            }
        }

        // All ok, so clone ref cards to create a new meld, then drain original Vec and return
        let meld = Run { cards: cards.into_iter().cloned().collect() };
        meld_cards.clear();
        Ok(meld)
    }

    fn layoff_card(&mut self, card: Card) -> Result<(), Card> {
        todo!();
    }
}