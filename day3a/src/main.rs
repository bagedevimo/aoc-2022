use std::{io,fs,env, collections::HashSet};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Specify input file");
    }

    let input_filename = &args[1];

    let input_data = fs::read_to_string(input_filename)?;

    let score = input_data.lines().fold(0, |accl, item| {
        accl + priority_for_rucksack(item)
    });

    println!("score: {}", score);

    Ok(())
}

fn priority_for_rucksack(line: &str) -> u32 {
    let len = line.len();
    let (first, second) = line.split_at(len / 2);
    let mut first_set = HashSet::new();
    let mut second_set = HashSet::new();
    first_set.extend(first.chars());
    second_set.extend(second.chars());

    let deltas = first_set.intersection(&second_set);
    
    deltas.fold(0, |accl, x| {
        accl + score_for(x)
    })
}

fn score_for(c: &char) -> u32 {
    match c {
        'A'..='Z' => (*c as u32) - 64 + 26,
        'a'..='z' => (*c as u32) - 96,
        _ => panic!("boopy"),
    }
}
