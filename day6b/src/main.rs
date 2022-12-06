use std::{io,fs,env, collections::HashSet};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("Specify input file");
    }

    let input_filename = &args[1];
    let input_data = fs::read_to_string(input_filename)?;

    for i in 14..input_data.len() {
        let sample = &input_data[i-14..i];
        let sample_set : HashSet<char> = sample.chars().collect();
        if sample_set.len() == 14 {
            println!("output: {}", i);
            break;
        }
    }

    Ok(())
}
