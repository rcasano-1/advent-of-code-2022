use std::collections::HashSet;

enum CommSelect {
    Packet = 4,
    Message = 14,
}

fn find_marker(input: &str, comm: CommSelect) -> Option<usize> {
    let retval = match comm {
        CommSelect::Packet => input
            .as_bytes()
            .windows(4)
            .position(|window| window.iter().collect::<HashSet<_>>().len() == 4)
            .map(|pos| pos + 4),
        CommSelect::Message => input
            .as_bytes()
            .windows(14)
            .position(|window| window.iter().collect::<HashSet<_>>().len() == 14)
            .map(|pos| pos + 14),
    };

    retval
}

fn main() {
    let input = include_str!("my_input.txt");
    let result = find_marker(input, CommSelect::Packet).unwrap();
    println!("The packet marker is at: {:?}", result);

    let result = find_marker(input, CommSelect::Message).unwrap();
    println!("The message marker is at: {:?}", result);
}
