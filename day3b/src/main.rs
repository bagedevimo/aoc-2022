use std::{io,fs,env, collections::{HashSet, HashMap}};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Specify input file");
    }

    let input_filename = &args[1];

    let input_data = fs::read_to_string(input_filename)?;

    let lines = input_data.lines().collect::<Vec<&str>>();
    let score = lines.chunks(3).fold(0, |accl, lines| {
        accl + priority_for_rucksack(lines.to_vec())
    });

    println!("score: {}", score);

    Ok(())
}

fn priority_for_rucksack(lines: Vec<&str>) -> u32 {
    let mut seen: HashMap<char, u32> = HashMap::new();
    let all_chars = lines
        .into_iter()
        .flat_map(|x| HashSet::<char>::from_iter(x.chars()));
    for c in all_chars {
        if !seen.contains_key(&c) {
            seen.insert(c, 0u32);
        }

        seen.insert(c, seen.get(&c).unwrap() + 1);
    }

    let best = seen.iter().max_by(|a, b| {
        a.1.cmp(b.1)
    });

    score_for(best.unwrap().0)
}

fn score_for(c: &char) -> u32 {
    match c {
        'A'..='Z' => (*c as u32) - 64 + 26,
        'a'..='z' => (*c as u32) - 96,
        _ => panic!("boopy"),
    }
}
