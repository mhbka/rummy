use super::{card::Card, suit_rank::Rank};


pub trait Meldable {
    /// Attempt to create a new meld out of `Card`s and indices of the chosen cards.
    /// 
    /// If valid, the indexed cards are removed and `Ok` is returned.
    /// Else, `Err` is returned and `meld_cards` is left untouched.
    fn new(cards: &mut Vec<Card>, indices: Vec<usize>) -> Result<Self, String> where Self: Sized;

    /// Attempt to add a card from `cards`, as chosen by `index`, to the meld.
    /// 
    /// If valid, the card is moved from `cards` into the meld and `Ok` is returned.
    /// 
    /// Else, `Error` is returned along with the card.
    fn layoff_card(&mut self, cards: &mut Vec<Card>, index: usize) -> Result<(), String>;
}


/// A Rummy meld.
/// There are 2 types: 
/// - **Set**; >=3 cards of same rank
/// - **Run**; >=3 sequential cards of same suit
#[derive(Debug)]
pub enum Meld {
    Set(Set),
    Run(Run)
}

impl Meldable for Meld {
    fn new(hand_cards: &mut Vec<Card>, indices: Vec<usize>) -> Result<Self, String> 
    where Self: Sized 
    {
        if let Ok(set) = Set::new(hand_cards, indices) {
            Ok(Meld::Set(set))
        }
        else if let Ok(run) = Run::new(hand_cards, indices) {
            Ok(Meld::Run(run))
        }
        else {
            Err("Cards don't form a valid set or run.".to_owned())
        }
    }

    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), String> {
        match self {
            Meld::Set(set) => set.layoff_card(hand_cards, index),
            Meld::Run(run) => run.layoff_card(hand_cards, index)
        }
    }
}


/// A Rummy meld set.
#[derive(Debug)]
pub struct Set {
    cards: Vec<Card>,
    pub(crate) set_rank: Rank
}

impl Meldable for Set {
    fn new(hand_cards: &mut Vec<Card>, indices: Vec<usize>) -> Result<Self, String> {
        let cards: Vec<&Card> = indices
            .iter()
            .map(|&i| {
                if i >= hand_cards.len() {
                    Err("Index in indices is greater than cards' size")
                }
                Ok(&hand_cards[i])
            })
            .cloned()
            .collect()?;
            

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
                        hand_cards // remove the meld cards from hand
                            .iter()
                            .enumerate()
                            .retain(|idx, card| !indices.contains(idx))
                            .collect();
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

    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), String> {
        let card = hand_cards
            .get(index)
            .ok_or("index is greater than hand_cards' size")?;
        
        if card.rank != self.set_rank { 
            return Err(card); 
        }
        else if let Some(wildcard_rank) = card.deck_config.wildcard_rank {
            if card.rank != wildcard_rank {
                return Err(card);
            }
        }

        self.cards.push(card);
        hand_cards.remove(index);
        Ok(())
    }
}


/// A Rummy meld run.
#[derive(Debug)]
pub struct Run {
    pub(crate) cards: Vec<Card>
}

impl Meldable for Run {
    fn new(hand_cards: &mut Vec<Card>, indices: Vec<usize>) -> Result<Self, String> {
        let cards: Vec<&Card> = indices
        .iter()
        .map(|&i| {
            if i >= hand_cards.len() {
                Err("Index in indices is greater than cards' size")
            }
            Ok(hand_cards[i])
        })
        .collect()?
        .cloned();

        let deck_config = cards[0].deck_config.clone();

        let (mut cards, mut wildcards) = match deck_config.wildcard_rank {
            Some(wildcard_rank) => cards.iter().partition(|&c| c.rank == wildcard_rank),
            None => cards.iter().collect(), Vec::new(),
        };

        // Check that each card is same suit and +1 rank from previous card (or previous card is wildcard).
        // If not, try to insert a wildcard and continue.
        // If we have no wildcards left to insert, return Err.
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

    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), String> {
        todo!();
    }
}