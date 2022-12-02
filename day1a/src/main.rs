use std::env;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("Specify input file");
    }

    let input_filename = &args[1];

    let input_data = fs::read_to_string(input_filename)?;
    let (mut elves, rem) = input_data.lines().fold((Vec::new(), 0), |(mut list, accl), line| {
        if let Ok(cals) = line.parse::<u32>() {
            (list, accl + cals)
        } else {
            list.push(accl);
            (list, 0)
        }
    });
    elves.push(rem);
    
    if let Some(max) = elves.iter().max() {
        println!("max: {}", max);
    }

    Ok(())
}
