use std::{io,fs,env};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("Specify input file");
    }

    let input_filename = &args[1];
    let input_data = fs::read_to_string(input_filename)?;

    let blank_line_index = input_data.lines().position(|x| {
        x == ""
    }).unwrap();

    let initial_state_lines = input_data
        .lines()
        .take(blank_line_index);

    let count_line = initial_state_lines
        .clone()
        .last()
        .unwrap()
        .split(" ")
        .fold(0, |accl, x| {
            if x != "" {
                let y = x.parse::<u32>().unwrap();
                if y > accl {
                    y
                } else {
                    accl
                }
            } else {
                accl
            }
        });

    let initial_state_rows = initial_state_lines
        .take(blank_line_index - 1)
        .map(|line| {
            let mut row = Vec::new();

            let mut chars = line.chars();

            loop {
                let nx = chars.next();

                match nx {
                    Some(nc) => {
                        if nc  == '[' {
                            let x = chars.next().unwrap();
                            row.push(x);
                            let x = chars.next();
                            let x = chars.next();
                        } else {
                            row.push(' ');
                            chars.next();
                            chars.next();
                            chars.next();
                        }
                    },
                    None => break,
                }
            }

            row
        })
        .collect::<Vec<Vec<char>>>();

    let mut stacks = Vec::new();
    for j in 0..count_line {
        let mut column = Vec::new();
        for i in 0..initial_state_rows.len() {
            let y : usize = initial_state_rows.len() - i - 1;
            let x : usize = j as usize;
            let item = initial_state_rows[y][x];
            if item != ' ' {
                column.push(item);
            }
        }
        stacks.push(column);
    }

    let instrs = input_data
        .lines()
        .skip(blank_line_index + 1)
        .collect::<Vec<&str>>();

    let final_state : Vec<Vec<char>> = instrs.iter().fold(stacks, |mut accl, instr| {
        println!("accl: {:?}", accl);
        println!("instr: {}", instr);

        if let ["move", qty, "from", src_stack_s, "to", dst_stack_s] = instr.split(" ").collect::<Vec<&str>>()[..] {
            let src_stack : usize = src_stack_s.parse::<usize>().unwrap() - 1;
            let dst_stack : usize = dst_stack_s.parse::<usize>().unwrap() - 1;

            let mut crane = Vec::new();
            for i in 0..qty.parse().unwrap() {
                crane.push(accl[src_stack].pop().unwrap());
            }

            for x in crane.iter().rev() {
                accl[dst_stack].push(*x);
            }
        } else {
            panic!("doesn't match!");
        }

        accl
    });

    let key: String = final_state.iter().map(|x| x[x.len() - 1] ).collect();

    println!("key: {:?}", key);

    Ok(())
}
