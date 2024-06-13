#[cfg(test)]

use std::rc::Rc;

use super::super::deck::DeckConfig;
use super::super::{
    card::Card, 
    deck::Deck, 
    meld::{Run, Set, Meld, Meldable},
    suit_rank::{Suit, Rank}
};

#[test]
/// Test the card permutations that would not form a run.
fn form_invalid_run() {
    let cfg = Rc::new(DeckConfig::new());

    // less than 3 cards would not be valid regardless
    let mut cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() }
    ];
    let mut indices = vec![0, 1];
    assert!(Run::new(&mut cards, &mut indices).is_err());

    // valid run of ranks but different suits would not be valid
    cards.push(Card { rank: Rank::Three, suit: Suit::Spades, deck_config: cfg.clone() });
    indices.push(2);
    assert!(Run::new(&mut cards, &mut indices).is_err());

    // valid run but without the proper indices would not be valid
    cards.push(Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() });
    assert!(Run::new(&mut cards, &mut indices).is_err());

    // if we set a `high_rank` in the deck config, the validity of a run would follow it
    let mut high_rank_cfg = DeckConfig::new();
    high_rank_cfg.high_rank = Some(Rank::Two);
    let high_rank_cfg = Rc::new(high_rank_cfg);
    cards = vec![
        Card { rank: Rank::King, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
    ];
    indices = vec![0, 1, 2, 3];
    assert!(Run::new(&mut cards, &mut indices).is_err()); // Two now highest, so Three is no longer consecutive
}

#[test]
/// Test the card permutations that would not form a run.
fn form_valid_run() {
    let cfg = Rc::new(DeckConfig::new());
    let mut cards = vec![
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() },
        Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() }
    ];
    let mut indices = vec![0, 1, 2];
    assert!(Run::new(&mut cards.clone(), &mut indices).is_ok());

    // valid even if the indices are in wrong order
    indices = vec![2, 0, 1];
    assert!(Run::new(&mut cards, &mut indices).is_ok());

    // if we use a custom `high_rank`, we can have different ordering for runs
    let mut high_rank_cfg = DeckConfig::new();
    high_rank_cfg.high_rank = Some(Rank::Two);
    let high_rank_cfg = Rc::new(high_rank_cfg);
    cards = vec![
        Card { rank: Rank::King, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
        Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
        Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: high_rank_cfg.clone() },
    ];
    indices = vec![0, 1, 2];
    match Run::new(&mut cards, &mut indices) {
        Err(err) => panic!("{err}"),
        Ok(_) => {}
    }
}

#[test]
/// Test the ability to layoff to a run.
fn layoff_run() {

}

