enum ParseState {
    Stacks,
    Instructions,
}

fn parse_crate(line: &str) -> Vec<char> {
    let mut it = line.chars();
    let mut crate_list = Vec::new();

    loop {
        let c = it.next().unwrap();
        if c == ' ' {
            it.next();
            it.next();
            crate_list.push('-');
        } else if c == '[' {
            crate_list.push(it.next().unwrap());
            it.next(); // Eat ']' of crate
        } else {
            panic!("Invalid Input.")
        }
        // Space or end of line check
        if it.next().is_none() {
            break;
        }
    }
    crate_list
}

fn main() {
    let input = include_str!("my_input.txt");
    let mut parse_state = ParseState::Stacks;
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        if line.starts_with(" 1") {
            for s in stacks.iter_mut() {
                s.reverse();
            }
            parse_state = ParseState::Instructions;
        } else if line.is_empty() {
        } else {
            match parse_state {
                ParseState::Stacks => {
                    let out = parse_crate(line);

                    // dbg!(stacks.len());
                    if stacks.len() == 0 {
                        stacks = vec![Vec::new(); out.len()];
                    }

                    for (i, c) in out.iter().enumerate() {
                        if *c != '-' {
                            stacks[i].push(*c);
                        }
                    }
                    // println!("{:?} Line: {}", out, i + 1);
                }
                ParseState::Instructions => {
                    let sections: Vec<_> = line.split(" ").collect();
                    let move_num = sections
                        .iter()
                        .skip(1)
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    let from = sections
                        .iter()
                        .skip(3)
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    let to = sections
                        .iter()
                        .skip(5)
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();

                    for _moves in 0..move_num {
                        let pop = stacks[from - 1].pop().unwrap();
                        stacks[to - 1].push(pop);
                    }
                }
            }
        }
    }

    for (i, s) in stacks.iter().enumerate() {
        println!("Stack {}: {}", i + 1, String::from_iter(s.iter()))
    }

    print!("Top crates: ");
    for s in stacks {
        print!("{}", s.last().unwrap());
    }
}
