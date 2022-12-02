use std::{io,fs,env};

enum Move {
    Rock,
    Paper,
    Scissors
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("Specify input file");
    }

    let input_filename = &args[1];

    let input_data = fs::read_to_string(input_filename)?;

    let score = input_data.lines().fold(0, |accl, x| {
        if let [opponent_move_raw, player_move_raw] = x.split(" ").collect::<Vec<&str>>()[..] {
            let opponent_move = raw_to_move(opponent_move_raw);
            let player_move = raw_to_move(player_move_raw);
            accl + score_for_turn(&opponent_move, &player_move) + score_for_shape(&player_move)
        } else {
            panic!("Wtf is {:?}", x);
        }
    });

    println!("score: {}", score);


    Ok(())
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
