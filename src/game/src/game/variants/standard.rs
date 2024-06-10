use std::collections::HashMap;
use crate::{
    cards::{
        card, deck::{Deck, DeckConfig}, meld::{
            Meld, 
            Meldable, 
            Run, 
            Set
        }, suit_rank::Rank::*
    }, game::state::{Score, State}, player::{self, Player}
};
use super::super::{
    actions::*,
    phases::*
};


/// State for a standard Rummy game.
type StandardRummyState = State<StandardRummyConfig, StandardRummyScore>;


/// Get the number of cards to deal each player at the start of a round,
/// given number of players and number of decks.
/// 
/// Follows the ruling [here](https://en.wikipedia.org/wiki/Rummy).
const fn get_cards_to_deal(num_players: usize, num_decks: usize) -> usize {
    match (num_players, num_decks) {
        (2, 1) => 10,
        (3, 1) => 7,
        (3, 2) => 10,
        (4..=5, 1) => 7,
        (4..=7, 2) => 10,
        (6, _) => 6,
        (7, _) => 6,
        _ => panic!("Invalid number of players or decks"),
    }
}


/// Entrypoint for starting a standard Rummy game.
pub struct StandardRummyGame();

impl StandardRummyGame {
    /// Start a new Rummy game with a list of `player_ids`, a game config, and a deck config.
    /// 
    /// If there are >7 players, the excess will be truncated.
    pub fn new(
        mut player_ids: Vec<usize>, 
        game_config: StandardRummyConfig, 
        deck_config: DeckConfig) 
    -> StandardRummy<RoundEndPhase> 
    {   
        player_ids.truncate(7);

        let players = player_ids
            .iter()
            .map(|&id| Player::new(id, true, 0))
            .collect();

        let state = StandardRummyState {
            config: game_config,
            score: StandardRummyScore::new(),
            deck: Deck::new(deck_config),
            players,
            cur_round: 0,
            cur_player: 0,
        };

        StandardRummy {
            phase: RoundEndPhase { has_scored_round: false },
            state
        }
    }

    /// Starts the game with default settings, only requiring a list of `player_ids`.
    /// 
    /// If there are >7 players, the excess will be truncated. 
    /// 
    /// If you want to configure your game, use `new` instead.
    pub fn quickstart(player_ids: Vec<usize>) -> StandardRummy<RoundEndPhase> {
        let deck_config = DeckConfig {
            shuffle_seed: None,
            pack_count: if player_ids.len() < 5 {1} else {2},
            use_joker: true,
            high_rank: None,
            wildcard_rank: None,
        };

        StandardRummyGame::new(
            player_ids,
            StandardRummyConfig::new(),
            deck_config
        )
    }  
}


/// Keeps the score of a standard Rummy game.
#[derive(Debug)]
pub struct StandardRummyScore { 
    score: HashMap<usize, HashMap<usize, usize>>
}

impl Score for StandardRummyScore {
    fn get(&self) -> &HashMap<usize, HashMap<usize, usize>> {
        &self.score
    }
}

impl StandardRummyScore {
    /// Initialize a new score struct.
    fn new() -> Self {
        StandardRummyScore { score: HashMap::new() }
    }

    /// Scores a set of players using the card values found [here](https://en.wikipedia.org/wiki/Rummy),
    /// and sets it for the current round.
    /// 
    /// If `score_winner_only`, all other players' hand's values will be added as the winner's score;
    /// else, each player is scored individually on their own hand's value.
    fn calculate(&mut self, scoreable_players: &Vec<&Player>, round: usize, score_winner_only: bool) {
        let individual_scores = StandardRummyScore::score_all(scoreable_players);

        let round_score = match self.score.get_mut(&round) {
            Some(round_score) => round_score,
            None => {
                self.score.insert(round, HashMap::new());
                self.score.get_mut(&round).unwrap()
            }
        };

        if !score_winner_only {
            for i in 0..scoreable_players.len() {
                round_score.insert(scoreable_players[i].id, individual_scores[i]);
            }
        }
        else {
            let winner_score = individual_scores
                .iter()
                .fold(0, |acc, &s| acc + s);
            let &winner = scoreable_players
                .iter()
                .find(|p| p.cards.len() == 0)
                .expect("The game must have a winner with 0 cards in hand");
            scoreable_players   
                .iter()
                .for_each(|&p| { // give winner his score, and everyone else 0
                    if std::ptr::eq(winner, p) {
                        round_score.insert(winner.id, winner_score);
                    }
                    else {
                        round_score.insert(p.id, 0);
                    }
                })
        }
    }

    /// Return a `Vec` where each element is the corresponding player's score.
    fn score_all(scoreable_players: &Vec<&Player>) -> Vec<usize> {
        scoreable_players
            .iter()
            .map(|&p| {
                p.cards
                    .iter()
                    .fold(0,|score, card| {
                        score + match card.rank {
                            Ace => 15,
                            King | Queen | Jack | Ten => 10,
                            Joker => 0,
                            rank => rank as usize,
                        }
                    })
            })
            .collect()
    }
}


/// The configurable options of a standard Rummy game.
#[derive(Debug)]
pub struct StandardRummyConfig {
    /// Whether only the winner is scored by the total of all other players' hands,
    /// 
    /// where the **overall winner has the highest score**,
    /// 
    /// or all players are scored by their own hand,
    /// 
    /// where the **overall winner has the lowest score**.
    pub score_winner_only: bool,

    /// Whether a player forfeits their cards and score if they quit, or keep the cards
    /// and get scored on the current state.
    pub forfeit_cards_on_quit: bool,

    /// Whether, once the deck stock is depleted and the discard pile is added into it,
    /// to shuffle the stock or just leave it turnt over.
    pub shuffle_stock_upon_depletion: bool,

    /// Whether or not to use a rank as a wildcard, which increases on each round.
    /// (for eg, Round 1 -> 2, Round 2 -> 3, Round 3 -> 4 ...)
    pub increasing_wildcard_rank: bool,

    /// How much of the discard pile can be drawn.
    /// - If `None`, the player can choose how many to draw.
    /// - If `Some(usize::MAX)`, the player must always take the entire discard pile.
    /// - Else, the player draws the specified amount (or the entire pile, if its size is smaller).
    pub discard_pile_draw_amount: Option<usize>,
}

impl StandardRummyConfig {
    /// Configure the game based on the rules [here](https://en.wikipedia.org/wiki/Rummy).
    /// 
    /// To initialize with your own settings, simply create this struct with its fields.
    pub fn new() -> Self {
        StandardRummyConfig {
            score_winner_only: true,
            forfeit_cards_on_quit: true,
            shuffle_stock_upon_depletion: false,
            increasing_wildcard_rank: false,
            discard_pile_draw_amount: Some(1)
        }
    }
}


/// A basic game of Rummy, following the rules/behaviour described [here](https://en.wikipedia.org/wiki/Rummy).
pub struct StandardRummy<P: GamePhase> {
    phase: P,
    state: StandardRummyState
}

impl <P: GamePhase> StandardRummy<P> {
    /// Returns a mutable reference to the current player.
    fn cur_player(&mut self) -> &mut Player {
        &mut self.state.players[self.state.cur_player]
    }

    /// Returns a reference to the config.
    fn config(&self) -> &StandardRummyConfig {
        &self.state.config
    }
}


impl DrawActions for StandardRummy<DrawPhase> {
    type SelfInPlayPhase = StandardRummy<PlayPhase>;

    fn draw_stock(&mut self) {
        let card = &mut self.state.deck
            .draw(1)
            .expect("Drawing 1 card should never cause an error"); // as we check and replenish below

        self.state
            .players[self.state.cur_player]
            .cards
            .append(card);
        
        if self.state
            .deck
            .stock().len() == 0 {
                self.state.deck.turnover_discarded();
            }

        self.phase.has_drawn = true;
    }

    fn draw_discard_pile(&mut self, amount: Option<usize>) -> Result<(), String> {
        self.state
            .players[self.state.cur_player]
            .cards
            .append(
                &mut self.state.deck.draw_discard_pile(amount)?
            );

        self.phase.has_drawn = true;

        Ok(())
    }

    fn to_play_phase(mut self) -> Self::SelfInPlayPhase {
        if !self.phase.has_drawn {
            self.draw_stock();
        }  
        StandardRummy {
            phase: PlayPhase { play_count: 0 },
            state: self.state
        }
    }
}


impl PlayActions for StandardRummy<PlayPhase> {
    type SelfInDiscardPhase = StandardRummy<DiscardPhase>;
    type SelfInRoundEndPhase = StandardRummy<RoundEndPhase>;

    fn form_meld(mut self, card_indices: Vec<usize>) 
    -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String>
    {
        if card_indices.len() < 3 {
            return TransitionResult::Error((
                self,
                "card_indices has less than 3 elements; need at least 3 for a meld".to_owned()
            ));
        }

        let player = &mut self.cur_player();

        if let Ok(meld) = Meld::new(&mut player.cards, card_indices) {
            player.melds.push(meld);
            return TransitionResult::Next(self);
        }
        else {
            return TransitionResult::Error((
                self,
                "Cards do not form a valid set or run".to_owned()
            ))
        }        
    }

    fn layoff_card(mut self, card_i: usize, target_player_i: usize, target_meld_i: usize)
    -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String>
    {
        let err_string;

        // check that all indices are valid first
        if card_i >= self.cur_player().cards.len() {
            err_string = "card_i is greater than current player's hand size";
        } 
        else if target_player_i >= self.state.players.len() {
            err_string = "target_player_i is greater than number of players";
        } 
        else if !self.state.players[target_player_i].active {
            err_string = "Target player is not active";
        } 
        else if target_meld_i >= self.state.players[target_player_i].melds.len() {
            err_string = "target_meld_i is greater than target player's number of melds";
        } 
        else {
            let meld = &mut self.state.players[target_player_i].melds[target_meld_i];

            match meld.layoff_card(&mut self.cur_player().cards, card_i) {
                Ok(_) =>{
                    if self.cur_player().cards.len() == 0 { // if all cards are gone, this player has won
                        return TransitionResult::End(
                            StandardRummy {
                                phase: RoundEndPhase { has_scored_round: false },
                                state: self.state
                            }
                        )
                    } 
                    else {
                        return TransitionResult::Next(self);
                    }
                },
                Err(err) => {
                    err_string = err.as_str();
                }
            }
        }

        TransitionResult::Error((
            self, 
            err_string.to_owned()
        ))
    }

    fn to_discard_phase(self) -> Self::SelfInDiscardPhase {
        StandardRummy {
            phase: DiscardPhase { has_discarded: false },
            state: self.state
        }
    }
}


impl DiscardActions for StandardRummy<DiscardPhase> {
    type SelfInDrawPhase = StandardRummy<DrawPhase>;
    type SelfInRoundEndPhase = StandardRummy<RoundEndPhase>;

    fn discard(mut self, card_i: usize) 
    -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String> 
    {
        if self.phase.has_discarded {
            return TransitionResult::Error((
                self,
                "Player has already discarded a card".to_owned()
            ));
        }

        let player_cards = &mut self.state
            .players[self.state.cur_player]
            .cards;

        let no_player_cards = player_cards.len();

        if card_i > no_player_cards {
            return TransitionResult::Error((
                self,
                format!("card_i ({}) is greater than player's hand size ({})", card_i, no_player_cards)
                ));
        }

        let card = player_cards.remove(card_i);
        self.state
            .deck
            .add_to_discard_pile(&mut vec![card]);

        if player_cards.len() == 0 {
            TransitionResult::End(
                StandardRummy {
                    phase: RoundEndPhase { has_scored_round: false },
                    state: self.state
                }
            )
        }
        else {
            self.phase.has_discarded = true;
            TransitionResult::Next(
                self
            )
        }
    }

    fn to_next_player(mut self)
    -> TransitionResult<Self::SelfInDrawPhase, Self::SelfInRoundEndPhase, Self, String> 
    {   
        // automatically discard the first card if discard hasn't been called yet,
        if !self.phase.has_discarded { 
            match self.discard(0) {
                TransitionResult::Next(s) => self = s,
                TransitionResult::End(e) => return TransitionResult::End(e),
                TransitionResult::Error(_) => unreachable!() // discarding first card should never error
            }
        }

        let mut state = self.state;
        
        // find the next active player
        state.cur_player = (state.cur_player + 1) % state.players.len();
        while !state.players[state.cur_player].active { 
            state.cur_player = (state.cur_player + 1) % state.players.len();
        }

        TransitionResult::Next(
            StandardRummy {
                phase: DrawPhase { has_drawn: false },
                state
            }
        )
    }
}


impl RoundEndActions for StandardRummy<RoundEndPhase> {
    type SelfInDrawPhase = StandardRummy<DrawPhase>;

    fn calculate_score(&mut self) {
        self.phase.has_scored_round = true;

        let scoreable_players = self.state.players
            .iter()
            .filter(|p| {
                // if forfeiting cards, only look at active players;
                // if not, look at all players with cards
                self.config().forfeit_cards_on_quit && p.active
                || !self.config().forfeit_cards_on_quit && p.cards.len() > 0
            })
            .collect();
            
        self.state.score.calculate(
            &scoreable_players, 
            self.state.cur_round, 
            self.config().score_winner_only)
    }

    fn to_next_round(mut self) -> Self::SelfInDrawPhase {
        if !self.phase.has_scored_round {
            self.calculate_score();
        }

        let mut state = self.state;
        state.deck.reset();

        // clear all players' cards, set players who just joined to active,
        // and tally up active players
        let mut num_active_players = 0;
        for player in &mut state.players {
            player.melds.clear();
            player.cards.clear();
            if player.joined_in_round == state.cur_round {
                player.active = true;
            }
            if player.active {
                num_active_players += 1;
            }
        }

        let num_deal_cards = get_cards_to_deal(
            num_active_players, 
            state.deck.config().pack_count
        );

        state.players
            .iter_mut()
            .filter(|p| p.active)
            .for_each(|p| {
                let mut deal_cards = state.deck
                    .draw(num_deal_cards)
                    .expect("Drawing pre-determined deal amounts should never cause an error");
                p.cards.append(&mut deal_cards);
            });

        state.cur_round += 1;

        StandardRummy {
            phase: DrawPhase { has_drawn: false },
            state
        }
    }
}


impl GameEndActions for StandardRummy<GameEndPhase> {

}

impl <P: GamePhase> AllActions<StandardRummyConfig, StandardRummyScore> for StandardRummy<P> {
    fn view_state(&self) -> &StandardRummyState {
        &self.state
    }
}

impl<P: GamePhase + PlayablePhase> PlayableActions for StandardRummy<P> {
    type SelfInRoundEndPhase = StandardRummy<RoundEndPhase>;
    type SelfInDrawPhase = StandardRummy<DrawPhase>;

    fn add_player(&mut self, player_id: usize, index: Option<usize>) -> Result<(), String> {
        if !self.state.players
            .iter()
            .all(|p| p.id != player_id)
        {
            return Err(format!("Player ID {player_id} already exists"));
        }

        let player = Player::new(player_id, false, self.state.cur_round);

        if index.is_none() || index.is_some_and(|i| i > self.state.players.len()) {
            self.state.players.push(player);
        }
        else if let Some(index) = index {
            self.state.players.insert(index, player);
        }

        Ok(())
    }

    fn quit_player(mut self, player_i: usize) 
    -> TransitionResult<Self, Self::SelfInRoundEndPhase, Self, String> 
    {
        if player_i == self.state.cur_player || player_i > self.state.players.len() {
            return TransitionResult::Error((
                self,
                format!("player_i {player_i} was either the current player, or greater than number of players")
            ));
        }

        self.cur_player().active = false;

        // end the round if there's only 1 player left
        if self.state.players
            .iter()
            .fold(0,|acc, p| acc + p.active as usize) <= 1 
        { 
            return TransitionResult::End(
                StandardRummy {
                    phase: RoundEndPhase { has_scored_round: false},
                    state: self.state
                }
            );
        }
        else {
            return TransitionResult::Next(self);
        }
    }
    
    fn quit_current_player(mut self) -> Self::SelfInDrawPhase {
        self.cur_player().active = false;

        let mut state = self.state;
        
        state.cur_player = (state.cur_player + 1) % state.players.len();
        while !state.players[state.cur_player].active { // find the next active player
            state.cur_player = (state.cur_player + 1) % state.players.len();
        }

        StandardRummy {
            phase: DrawPhase { has_drawn: true },
            state
        }
    }

    fn move_card_in_hand(&mut self, player_i: usize, old_pos: usize, mut new_pos: usize) 
    -> Result<(), String> 
    {
        if player_i > self.state.players.len() {
            return Err(format!("player_i {player_i} is greater than number of players"));
        }
        
        let player_hand = &mut self.state.players[player_i].cards;
        if old_pos > player_hand.len() {
            return Err(format!("old_pos {old_pos} is greater than the player's hand's size"));
        }

        if new_pos > player_hand.len() {
            new_pos = player_hand.len() - 1;
        }

        let card = player_hand.remove(old_pos);
        player_hand.insert(new_pos - 1, card); 

        Ok(())
    }
}