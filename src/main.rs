extern crate rock_paper_scissors;

use rock_paper_scissors::game::{play, player};
use rock_paper_scissors::util::{input, output};

enum Result<'a> {
    WIN(&'a mut player::Player),
    DRAW,
}

fn main() {
    println!("Welcome to RockPaperScissors!");
    output::print_flush("How many games would you like to play? ");
    let mut games: usize = input::read_line().parse().unwrap();
    while games % 2 == 0 {
        output::print_flush("The number must be odd! How many games would you like to play? ");
        games = input::read_line().parse().unwrap();
    }

    //V is mutable to have the variables so
    let mut computer: player::Player = player::Player::new("Computer");
    let mut user: player::Player = player::Player::new("You");
    for x in 0..games {
        println!("--------------- [ ROUND {} ] ---------------", x+1);
        loop {
            let user_play: play::Type = parse_play();
            let computer_play: play::Type = play::random_play();
            let mut result: Result = get_result(play::Play { choice: user_play, player: &mut user }, play::Play { choice: computer_play, player: &mut computer });
            match result {
                Result::DRAW => {
                    println!("It's a tie!");
                    continue
                },
                Result::WIN(ref mut player) => {
                    player.increment_score();
                    println!("{} win{}!", &player.name, if &player.name != "You" { "s" } else { "" });
                    break
                }
            }
        }
    }

    println!();
    println!("You had {} {}", user.score, output::format_count("win".to_string(), user.score));
    println!("The computer had {} {}", computer.score, output::format_count("win".to_string(), computer.score));
    let winner: player::Player = if user.score > computer.score { user } else { computer };
    println!("Overall, {} win{}", &winner.name, if &winner.name != "You" { "s" } else { "" });
}

fn parse_play() -> play::Type {
    output::print_flush("What play do you want to make? (rock, paper, scissors) ");
    match play::parse_type(input::read_line()) {
        Some(x) => x,
        None => {
            println!("Invalid value.");
            parse_play()
        }
    }
}

fn get_result<'a>(play_one: play::Play<'a>, play_two: play::Play<'a>) -> Result<'a> {
    let user_beats = play_one.choice.get_beats();
    let computer_beats = play_two.choice.get_beats();
    if user_beats.contains(&play_two.choice) {
        Result::WIN(play_one.player)
    } else if computer_beats.contains(&play_one.choice) {
        Result::WIN(play_two.player)
    } else {
        Result::DRAW
    }
}
