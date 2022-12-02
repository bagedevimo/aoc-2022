use std::{io,fs,env};

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("Specify input file");
    }

    let input_filename = &args[1];

    let input_data = fs::read_to_string(input_filename)?;

    let score = input_data.lines().fold(0, |accl, x| {
        if let [opponent_move_raw, desired_outcome_raw] = x.split(" ").collect::<Vec<&str>>()[..] {
            let opponent_move = raw_to_move(opponent_move_raw);

            let desired_outcome = match desired_outcome_raw {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => panic!("wtf is a {:?}", desired_outcome_raw),
            };

            let player_move = move_for_outcome(&opponent_move, &desired_outcome);
            let score_turn = score_for_turn(&opponent_move, &player_move);
            let score_shape = score_for_shape(&player_move);
            accl + score_turn + score_shape
        } else {
            panic!("Wtf is {:?}", x);
        }
    });

    println!("score: {}", score);


    Ok(())
}

fn move_for_outcome(opponent_move: &Move, desired_outcome: &Outcome) -> Move {
    match (opponent_move, desired_outcome) {
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Rock, Outcome::Draw) => Move::Rock,
        (Move::Rock, Outcome::Lose) => Move::Scissors,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Paper, Outcome::Draw) => Move::Paper,
        (Move::Paper, Outcome::Lose) => Move::Rock,
        (Move::Scissors, Outcome::Win) => Move::Rock,
        (Move::Scissors, Outcome::Draw) => Move::Scissors,
        (Move::Scissors, Outcome::Lose) => Move::Paper,
    }
}

fn raw_to_move(this_move: &str) -> Move {
    match this_move {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!("Wtf is a {:?}", this_move),
    }
}

fn score_for_turn(opponent_move: &Move, player_move: &Move) -> u32 {
    match (opponent_move, player_move) {
        (Move::Rock, Move::Rock) => 3,
        (Move::Rock, Move::Paper) => 6,
        (Move::Rock, Move::Scissors) => 0,
        (Move::Paper, Move::Rock) => 0,
        (Move::Paper, Move::Paper) => 3,
        (Move::Paper, Move::Scissors) => 6,
        (Move::Scissors, Move::Rock) => 6,
        (Move::Scissors, Move::Paper) => 0,
        (Move::Scissors, Move::Scissors) => 3,
    }
}

fn score_for_shape(player_move: &Move) -> u32 {
    match player_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}
