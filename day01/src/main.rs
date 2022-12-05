fn main() {
    let input = include_str!("input.txt");

    let mut curr_elf: Vec<u64> = Vec::new();
    let mut curr_max = 0;

    for line in input.lines() {
        if line.is_empty() {
            let elf_sum = curr_elf.iter().sum();
            if elf_sum > curr_max {
                curr_max = elf_sum;
            }
            curr_elf.clear();
        } else {
            let value: u64 = line.parse().unwrap();
            curr_elf.push(value);
        }
    }

    let elf_sum = curr_elf.iter().sum();
    if elf_sum > curr_max {
        curr_max = elf_sum;
    }
    curr_elf.clear();

    println!("Single Elf Max: {curr_max}");

    let mut elf_sums: Vec<u64> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            let elf_sum = curr_elf.iter().sum();
            elf_sums.push(elf_sum);
            curr_elf.clear();
        } else {
            let value: u64 = line.parse().unwrap();
            curr_elf.push(value);
        }
    }

    let elf_sum = curr_elf.iter().sum();
    elf_sums.push(elf_sum);
    curr_elf.clear();

    elf_sums.sort();
    elf_sums.reverse();

    let total: u64 = elf_sums[..3].iter().sum();

    println!("Top Three Elf's Total: {total}");
    println!("{:?}", &elf_sums[..3]);

    ////////////////////////////////////////////////////////////////////////////

    let mut elves: Vec<Box<Vec<u64>>> = Vec::new();
    let mut curr_elf: Box<Vec<u64>> = Box::new(Vec::new());
    for line in input.lines() {
        if line.is_empty() {
            elves.push(curr_elf);
            curr_elf = Box::new(Vec::new());
        } else {
            let value: u64 = line.parse().unwrap();
            curr_elf.push(value);
        }
    }
    elves.push(curr_elf);
    println!("All the elves: {:?}", elves);

    let mut sums: Vec<u64> = elves.iter().map(|e| e.iter().sum()).collect();
    sums.sort();
    sums.reverse();

    println!("Max Elf: {}", sums[0]);

    let total: u64 = sums[..3].iter().sum();
    println!("Top 3 Total: {total}");

    ////////////////////////////////////////////////////////////////////////////

    let mut sums: Vec<u64> = input
        .lines()
        .collect::<Vec<_>>()
        .split(|l| l.is_empty())
        .map(|e| {
            e.iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|e| e.iter().sum())
        .collect();

    sums.sort();
    sums.reverse();

    println!("Max Elf: {}", sums[0]);

    let total: u64 = sums[..3].iter().sum();
    println!("Top 3 Total: {total}");

    ///////////////////////////////////////////////////////////////////////////

    let mut sums = input
        .lines()
        .map(|line| line.parse::<u64>().ok())
        .collect::<Vec<_>>()
        .split(Option::is_none)
        .map(|e| e.iter().map(|i| i.unwrap()).collect::<Vec<_>>())
        .map(|e| e.iter().sum::<u64>())
        .collect::<Vec<_>>();

    println!("{:?}", sums);

    sums.sort();
    sums.reverse();

    println!("Max Elf: {}", sums[0]);

    let total: u64 = sums[..3].iter().sum();
    println!("Top 3 Total: {total}");
}
