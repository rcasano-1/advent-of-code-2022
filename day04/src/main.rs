fn main() {
    let input = include_str!("my_input.txt");
    let mut sum_contains = 0;
    let mut sum_overlap = 0;

    for line in input.lines() {
        let (lhs, rhs) = line.split_once(',').unwrap();
        let lhs = lhs.split_once('-').unwrap();
        let rhs = rhs.split_once('-').unwrap();
        let lhs: (u64, u64) = (lhs.0.parse().unwrap(), lhs.1.parse().unwrap());
        let rhs: (u64, u64) = (rhs.0.parse().unwrap(), rhs.1.parse().unwrap());

        if (lhs.0 <= rhs.0 && lhs.1 >= rhs.1) || (lhs.0 >= rhs.0 && lhs.1 <= rhs.1) {
            sum_contains += 1;
        }

        let first_starts_in_second = lhs.0 >= rhs.0 && lhs.0 <= rhs.1;
        let first_ends_in_second = lhs.1 >= rhs.0 && lhs.1 <= rhs.1;
        let second_starts_in_first = rhs.0 <= lhs.1 && rhs.0 >= lhs.0;
        let second_ends_in_first = rhs.1 <= lhs.1 && rhs.1 >= lhs.0;

        let overlap = first_starts_in_second
            || first_ends_in_second
            || second_starts_in_first
            || second_ends_in_first;
        if overlap {
            sum_overlap += 1;
        }
    }
    println!("{sum_contains}");
    println!("{sum_overlap}");
}
