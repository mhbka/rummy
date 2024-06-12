#[cfg(test)]

mod card {
    use std::rc::Rc;
    use super::deck::DeckConfig;
    use super::{card::Card, 
        deck::Deck, 
        suit_rank::{Suit, Rank}
    };

    #[test]
    /// Cards have the expected ordering.
    fn normal_ordering_card() {
        let cfg = Rc::new(DeckConfig::new());
    
        // cards are ordered by rank, then suit
        let card1 = Card { rank: Rank::Ace, suit: Suit::Clubs, deck_config: cfg.clone() };
        let card2 = Card { rank: Rank::Ace, suit: Suit::Diamonds, deck_config: cfg.clone() };
        let card3 = Card { rank: Rank::Two, suit: Suit::Clubs, deck_config: cfg.clone() };
        
        assert!(card2 > card1);
        assert!(card3 > card2);
    }
    
    #[test]
    /// If the deck config specifies a custom high rank,
    /// ordering will decrease circularly from that rank.
    /// 
    /// For eg, `high_rank = 3` means `3 > 2 > Ace > King > Queen > ...` 
    fn custom_ordering_card() {
        let cfg = Rc::new(DeckConfig {
            shuffle_seed: None,
            pack_count: 1,
            use_joker: false,
            high_rank: Some(Rank::Three),
            wildcard_rank: None,
        });
    
        // Rank::Three should be the highest now
        let card1 = Card { rank: Rank::King, suit: Suit::Spades, deck_config: cfg.clone() };
        let card2 = Card { rank: Rank::Two, suit: Suit::Spades, deck_config: cfg.clone() };
        let card3 = Card { rank: Rank::Three, suit: Suit::Clubs, deck_config: cfg.clone() };
    
        assert!(card2 > card1);
        assert!(card3 > card2);
    
        // Suit ordering should remain the same
        let card4 = Card { rank: Rank::Three, suit: Suit::Spades, deck_config: cfg.clone() };
        assert!(card4 > card1);
    }
}


mod deck {
    use std::rc::Rc;
    use super::deck::DeckConfig;
    use super::{card::Card, 
        deck::Deck, 
        suit_rank::{Suit, Rank}
    };
    
    /// Normal deck must be instantiated correctly.
    #[test]
    fn normal_deck() {
        let mut cfg = DeckConfig {
            shuffle_seed: None,
            pack_count: 1,
            use_joker: false,
            high_rank: None,
            wildcard_rank: None,
        };

        let default_cfg = DeckConfig::new();
        assert_eq!(cfg, default_cfg);

        let deck = Deck::new(cfg.clone());
        assert_eq!(deck.stock().len(), 52);
        assert_eq!(deck.discard_pile().len(), 0);

        // `use_joker` should add 2 jokers to the deck
        cfg.use_joker = true;
        let joker_deck = Deck::new(cfg.clone());
        assert_eq!(joker_deck.stock().len(), 54);

        // `pack_count = 0` should be set to 1 during Deck::new()
        cfg.pack_count = 0; 
        let zero_pack_deck = Deck::new(cfg.clone());
        assert_eq!(zero_pack_deck.config().pack_count, 1);
        assert_eq!(zero_pack_deck.stock().len(), 54);
    }

    /// Two pack deck must be instantiated correctly.
    #[test]
    fn two_pack_deck() {
        let mut cfg = DeckConfig {
            shuffle_seed: None,
            pack_count: 2,
            use_joker: false,
            high_rank: None,
            wildcard_rank: None,
        };

        let deck = Deck::new(cfg.clone());
        assert_eq!(deck.stock().len(), 104);
        assert_eq!(deck.discard_pile().len(), 0);

        cfg.use_joker = true;
        let joker_deck = Deck::new(cfg.clone());
        assert_eq!(joker_deck.stock().len(), 108);
    }

    /// If the shuffle seed is `Some(0)`, the stock shouldn't be shuffled;
    /// ie, always in increasing order.
    #[test]
    fn no_shuffle_deck() {
        let cfg = DeckConfig {
            shuffle_seed: Some(0),
            pack_count: 1,
            use_joker: false,
            high_rank: None,
            wildcard_rank: None,
        };

        let deck = Deck::new(cfg.clone());
        assert!(deck.stock()
            .windows(2)
            .all(|w| w[0] <= w[1])
        );
    }

    /// Draw and discard must work as expected.
    #[test]
    fn draw_and_discard_deck() {
        let mut deck = Deck::new(DeckConfig::new());

        // Drawing 1 card
        let mut card = deck.draw(1).unwrap();
        deck.add_to_discard_pile(&mut card);
        assert_eq!(deck.stock().len(), 51);
        assert_eq!(deck.discard_pile().len(), 1);

        // Drawing several cards
        let mut cards = deck.draw(51).unwrap();
        deck.add_to_discard_pile(&mut cards);
        assert_eq!(deck.stock().len(), 0);
        assert_eq!(deck.discard_pile().len(), 52);

        // Drawing from an empty stock
        assert!(deck.draw(1).is_err());

        // `shuffle_discarded` replenishes the stock but shuffles it
        deck.shuffle_discarded();
        assert_eq!(deck.stock().len(), 52);
        assert_eq!(deck.discard_pile().len(), 0);

        // `turnover_discarded` replenishes the stock and preserves order (as no shuffle occurs)
        // we can test by creating an unshuffled deck, calling `turnover_discarded` and checking the order
        let mut unshuffled_deck = Deck::new(
            DeckConfig {
                shuffle_seed: Some(0),
                pack_count: 1,
                use_joker: false,
                high_rank: None,
                wildcard_rank: None
            }
        );
        let mut cards = unshuffled_deck.draw(52).unwrap();
        unshuffled_deck.add_to_discard_pile(&mut cards);
        unshuffled_deck.turnover_discarded();
        println!("{:?}", unshuffled_deck.stock());
        assert_eq!(unshuffled_deck.stock().len(), 52);
        assert_eq!(unshuffled_deck.discard_pile().len(), 0);
        assert!(unshuffled_deck.stock()
            .windows(2)
            .all(|w| w[0] >= w[1])
        );
    }
}

mod meld {
    use std::rc::Rc;
    use super::deck::DeckConfig;
    use super::{card::Card, 
        deck::Deck, 
        suit_rank::{Suit, Rank}
    };

    #[test]
    
}


