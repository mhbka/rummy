pub mod cards;
pub mod game;
pub mod player;

use rprompt;
use game::{
    actions::{
        AllActions, DiscardActions, DrawActions, PlayActions, PlayableActions, RoundEndActions, TransitionResult
    }, phases::{DiscardPhase, DrawPhase, PlayPhase, RoundEndPhase}, state::{Score, State}, variants::standard::{
        StandardRummy, 
        StandardRummyGame
    }
};

fn main() {
    let player_ids = vec![1, 2, 3, 4];
    let mut game = StandardRummyGame::quickstart(player_ids);
    handle_round(game);
}

fn handle_round(game: StandardRummy<RoundEndPhase>) -> StandardRummy<RoundEndPhase> {
    let mut game = game.to_next_round();

    handle_draw(&mut game);

    let game = game.to_play_phase();

    let game = match handle_play(game) {
        Ok(game) => game,
        Err(game) => {
            println!("Round has ended; returning...");
            return game; 
        }
    };

    let game = game.to_discard_phase();

    let game = match handle_discard(game) {
        Ok(game) => game,
        Err(game) => {
            println!("Round has ended; returning...");
            return game; 
        }
    }
}

fn print_state<C, S: Score>(state: &State<C, S>) {
    println!("Current player: {}", state.players[state.cur_player].id());
    println!("Deck size: {}", state.deck.stock().len());
    println!("Top discard card and size: {:?}, {}", state.deck.peek_discard_pile(), state.deck.discard_pile().len());
    println!("Players: ");

    for player in &state.players {
        println!("---------------");

        println!("Hand: {:?}", player.cards());
        println!("----");

        println!("Melds: ");    
        for meld in player.melds {
            println!("{meld:?}");
        }
        println!("----");

        println!("---------------");
    }
}

fn handle_draw(game: &mut StandardRummy<DrawPhase>) {
    let state = game.view_state();

    print_state(state);

    if state.deck.discard_pile().len() == 0 {
        println!("No discard pile, drawing from stock...");
        game.draw_stock();
    }

    match rprompt::prompt_reply(r#"
        1. Draw stock
        2. Attempt to draw from discard pile
    "#)
        .unwrap()
        .as_str() {
        "1" => game.draw_stock(),
        "2" => {
            let amount = rprompt::prompt_reply("Draw how many?: ")
                .unwrap()
                .parse()
                .ok();
            if game.draw_discard_pile(amount).is_err() {
                println!("Not enough cards in pile; drawing from stock...");
                game.draw_stock();
            }
        },
        _ => {
            println!("Invalid; drawing from stock...");
            game.draw_stock();
        }
    }
}

fn handle_play(game: StandardRummy<PlayPhase>) -> Result<StandardRummy<PlayPhase>, StandardRummy<RoundEndPhase>>
{
    let state = game.view_state();
    print_state(state);

    loop {
        let play_result = match rprompt::prompt_reply(r#"
        1. Form meld
        2. Layoff card
        3. Discard
        "#)
            .unwrap()
            .as_str() {
            "1" => {
                play_meld(game)
            },
            "2" => {
                play_layoff(game)
            },
            "3" => {
                println!("Continuing...");
                TransitionResult::Next(game)
            },
            _ => {
                println!("Invalid input, continuing...");
                TransitionResult::Next(game)
            }
        };

        let game = match play_result {
            TransitionResult::Next(game) => game,
            TransitionResult::End(game) => return Err(game),
            TransitionResult::Error(res) => res.0
        };

        match rprompt::prompt_reply("Try again? (Y/N): ").unwrap().as_str() {
            "Y" | "y" => continue,
            "N" | "n" => return Ok(game),
            _ => {
                println!("Not valid input; going to discard...");
                return Ok(game);
            }
        }
    };
}

fn play_meld(game: StandardRummy<PlayPhase>)
-> TransitionResult<StandardRummy<PlayPhase>, StandardRummy<RoundEndPhase>, StandardRummy<PlayPhase>, String> 
{
    let cur_player = &game.view_state().players[game.view_state().cur_player];
    let mut indices = Vec::new();
    let mut index = 0;

    loop {
        match rprompt::prompt_reply("Enter index of card to put in meld (-1 to stop): ")
            .unwrap()
            .parse() 
        {
            Ok(i) => {
                if i < 0 {
                    println!("Collecting...");
                    break;
                }
                else if i > cur_player.cards.len() {
                    println!("Greater than player's hand size. Try again.");
                }
                else {
                    println!("Chosen card: {:?}", cur_player.cards[i]);
                    indices.push(i);
                }
            },
            Err(_) => println!("Invalid input.")
        }
    }

    game.form_meld(indices)
}

fn play_layoff(game: StandardRummy<PlayPhase>) 
-> TransitionResult<StandardRummy<PlayPhase>, StandardRummy<RoundEndPhase>, StandardRummy<PlayPhase>, String> 
{
    let cur_player = &game.view_state().players[game.view_state().cur_player];

    let card_i = match rprompt::prompt_reply("Enter index of card to layoff: ")
        .unwrap()
        .parse()
    {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid input; returning...");
            return TransitionResult::Next(game);
        },
    }

    let target_player_i = match rprompt::prompt_reply("Enter index of targeted player: ")
        .unwrap()
        .parse()    
    {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid input; returning...");
            return TransitionResult::Next(game);
        },
    };

    let target_meld_i = match rprompt::prompt_reply("Enter index of targeted meld: ")
        .unwrap()
        .parse()    
    {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid input; returning...");
            return TransitionResult::Next(game);
        },
    };

    game.layoff_card(card_i, target_player_i, target_meld_i)
}

fn handle_discard(game: StandardRummy<DiscardPhase>) -> Result< StandardRummy<DiscardPhase>, StandardRummy<RoundEndPhase>> {
    let state = game.view_state();

    print_state(state);

    loop {
        let i = match rprompt::prompt_reply("Choose a card to discard: ")
            .unwrap()
            .parse() {
                Ok(i) => i,
                Err(_) => {
                    println!("Invalid; try again...");
                    continue;
                }
            };
        
        game.discard(i)
    }
}