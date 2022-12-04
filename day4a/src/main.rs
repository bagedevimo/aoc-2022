use std::{io,fs,env};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("Specify input file");
    }

    let input_filename = &args[1];
    let input_data = fs::read_to_string(input_filename)?;

    let score = input_data.lines().fold(0, |accl, line| {
        if let [elf1, elf2] = split_parts(line, ',')[..] {
            if let [elf1_min_str, elf1_max_str] = split_parts(elf1, '-')[..] {
                if let [elf2_min_str, elf2_max_str] = split_parts(elf2, '-')[..] {
                    let elf1_min = elf1_min_str.parse::<u32>().unwrap();
                    let elf1_max = elf1_max_str.parse::<u32>().unwrap();
                    let elf2_min = elf2_min_str.parse::<u32>().unwrap();
                    let elf2_max = elf2_max_str.parse::<u32>().unwrap();
                    
                    // does elf1 fit into elf2?
                    if elf1_min >= elf2_min && elf1_max <= elf2_max {
                        return accl + 1;
                    // does elf2 fit into elf1?
                    } else if elf2_min >= elf1_min && elf2_max <= elf1_max {
                        return accl + 1;
                    } 
                }
            }
        }

        accl
    });

    println!("score: {}", score);

    Ok(())
}

fn split_parts<'a>(input: &'a str, c: char) -> Vec<&'a str> {
    input.split(c).collect::<Vec<&str>>()
}
