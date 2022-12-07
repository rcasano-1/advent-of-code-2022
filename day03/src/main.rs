fn priority(item: char) -> u64 {
    match item {
        // Convert from ASCII and shift priority by 1 (lowercase) or 27 (uppercase)
        'a'..='z' => 1 + (item as u64) - ('a' as u64),
        'A'..='Z' => 27 + (item as u64) - ('A' as u64),
        _ => panic!("Invalid character!"),
    }
}

fn main() {
    let input = include_str!("my_input.txt");

    let mut sum_priority = 0;
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let match_found = second.chars().find(|m| first.contains(*m)).unwrap();
        println!("Found duplicate item: {match_found}");
        sum_priority += priority(match_found);
    }
    println!("The total priority value of the duplicate items is: {sum_priority}");

    let mut sum_badges = 0;
    for line in input.lines().collect::<Vec<_>>().chunks(3) {
        let first = line[0];
        let second = line[1];
        let third = line[2];

        let badge_found = third
            .chars()
            .find(|b| first.contains(*b) && second.contains(*b))
            .unwrap();

        println!("Found bagdge: {badge_found}");
        sum_badges += priority(badge_found);
    }
    println!("The total priority value of the badges is: {sum_badges}");
}
