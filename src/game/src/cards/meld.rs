use super::{card::Card, suit_rank::{Rank, Suit}};


pub trait Meldable {
    /// Attempt to create a new meld out of `Card`s and indices of the chosen cards.
    /// 
    /// If valid, the indexed cards are removed and `Ok` is returned.
    /// Else, `Err` is returned and `meld_cards` is left untouched.
    fn new(hand_cards: &mut Vec<Card>, indices: &Vec<usize>) -> Result<Self, String> where Self: Sized;

    /// Attempt to add a card from `cards`, as chosen by `index`, to the meld.
    /// 
    /// If valid, the card is moved from `cards` into the meld and `Ok` is returned.
    /// 
    /// Else, `Error` is returned along with the card.
    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), String>;
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
    fn new(hand_cards: &mut Vec<Card>, indices: &Vec<usize>) -> Result<Self, String> 
    where Self: Sized 
    {
        match Set::new(hand_cards, indices) {
            Ok(set) => Ok(Meld::Set(set)),
            Err(set_err) => {
                match Run::new(hand_cards, indices) {
                    Ok(run) => Ok(Meld::Run(run)),
                    Err(run_err) => {
                        Err(format!("Couldn't form set ({set_err}) or run ({run_err})"))
                    }
                }
            }
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
    set_rank: Rank
}

impl Meldable for Set {
    fn new(hand_cards: &mut Vec<Card>, indices: &Vec<usize>) -> Result<Self, String> {
        let cards = indices
            .iter()
            .map(|&i| {
                hand_cards
                    .get(i)
                    .ok_or("index is greater than hand_cards size".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        
        match cards[0].deck_config.wildcard_rank {
            // check if every card has same rank, or the wildcard rank
            Some(wildcard_rank) => {
                let mut set_rank: Option<Rank> = None;
                if cards
                    .iter()
                    .all(|card| {
                        if card.rank == wildcard_rank { 
                            return true; 
                        }
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
                        let cards = cards.into_iter().cloned().collect();
                        return Ok(Set{ set_rank, cards });
                    }
                    // every card is a wildcard, which is not a valid set.
                    else { 
                        return Err("A set cannot be formed out of only wildcards".into());
                    }
                }
                else {
                    return Err("Cards do not form a valid set".into());
                }
            },
            
            // we check if every card has same rank
            None => { 
                if cards
                    .iter()
                    .all(|card| card.rank == cards[0].rank) {
                        let cards: Vec<_> = cards // clone meld cards into a new vec
                            .into_iter()
                            .cloned()
                            .collect();
                    
                        let mut idx = 0;
                        hand_cards.retain(|_| { // remove meld cards from hand
                                idx += 1;
                                !indices.contains(&(idx - 1))
                            });

                        return Ok(
                            Set{ set_rank: cards[0].rank, cards }
                        );
                }   
                else {
                    return Err("Cards do not form a valid set".into());
                }
            }
        }
    }

    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), String> {
        let card = hand_cards
            .get(index)
            .ok_or("index is greater than hand_cards' size")?;
        
        if card.rank != self.set_rank { 
            return Err("Card rank is not same as set's rank".to_string()); 
        }
        else if let Some(wildcard_rank) = card.deck_config.wildcard_rank {
            if card.rank != wildcard_rank {
                return Err("Card rank is not same as set's rank or wildcard rank".to_string());
            }
        }

        self.cards.push(
            hand_cards.remove(index)
        );

        Ok(())
    }
}


/// A Rummy meld run.
#[derive(Debug)]
pub struct Run {
    cards: Vec<Card>,
    suit: Suit
}

impl Meldable for Run {
    fn new(hand_cards: &mut Vec<Card>, indices: &Vec<usize>) -> Result<Self, String> {
        let cards = indices
            .iter()
            .map(|&i| {
                hand_cards.get(i)
                    .ok_or("Index in indices is greater than cards' size")
            })
            .collect::<Result<Vec<_>, _>>()?;

        let deck_config = cards[0].deck_config.clone();

        let (mut cards, mut wildcards) = match deck_config.wildcard_rank {
            Some(wildcard_rank) => cards.iter().partition(|&c| c.rank == wildcard_rank),
            None => (cards.iter().collect(), Vec::new())
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
                return Err("Cards don't form a valid run".into());
            }
        }

        let cards: Vec<_> = cards
            .iter()
            .map(|&&c| c) 
            .cloned()
            .collect();

        let suit = cards[0].suit;

        let mut idx = 0;
        hand_cards.retain(|_| {
            idx += 1;
            indices.contains(&(idx - 1))
        });

        Ok(Run { cards, suit })
    }

    fn layoff_card(&mut self, hand_cards: &mut Vec<Card>, index: usize) -> Result<(), String> {
        let card = hand_cards
            .get(index)
            .ok_or("index is greater than hand_cards' size")?;

        if card.suit != self.suit {
            return Err("Card's suit isn't same as run's suit".into());
        }
        else if let Some(wildcard_rank) = card.deck_config.wildcard_rank {
            if card.rank == wildcard_rank {
                self.cards.push(
                    hand_cards.remove(index)
                );
                return Ok(());
            }
        }
        else {
            for (idx, &ref meld_card) in self.cards.iter().enumerate() {
                if card.rank as u8 + 1 == meld_card.rank as u8 {
                    self.cards.insert(idx, hand_cards.remove(index));
                    return Ok(());
                }
                else if card.rank as u8 -1 == meld_card.rank as u8 {
                    self.cards.insert(idx + 1, hand_cards.remove(index));
                    return Ok(());
                }
            }
        }
        
        Err("Card cannot be laid off in this run".into())
    }
}