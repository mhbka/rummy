pub mod cards;
pub mod game;
pub mod player;

use rprompt;
use game::{
    actions::{
        AllActions, DrawActions, PlayActions, PlayableActions, RoundEndActions, TransitionResult
    }, phases::{DrawPhase, PlayPhase, RoundEndPhase}, state::{Score, State}, variants::standard::{
        StandardRummy, 
        StandardRummyGame
    }
};

fn main() {
    let player_ids = vec![1, 2, 3, 4];
    let mut game = StandardRummyGame::quickstart(player_ids);
    handle_round(game);
}

fn handle_round(game: StandardRummy<RoundEndPhase>) {
    let mut game = game.to_next_round();
    handle_draw(&mut game);

    let mut game = game.to_play_phase();
    handle_play(game);
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

fn handle_play(game: StandardRummy<PlayPhase>) 
-> TransitionResult<Self, StandardRummy<RoundEndPhase>, Self, String> 
{
    let state = game.view_state();
    print_state(state);

    match rprompt::prompt_reply(r#"
        1. Form meld
        2. Layoff card
        3. Continue
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
    }
}

fn play_meld(game: StandardRummy<PlayPhase>) {
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
            }
        }
    }

    indices
        .iter()
        .map(|i| )
}

fn play_layoff(game: StandardRummy<PlayPhase>) {

}