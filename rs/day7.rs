use std::collections::HashMap;

fn parse_input(input: &str)  -> HashMap<u32, u32> {
    let mut qwe: HashMap<u32, u32> = HashMap::new();

    for pos in input.split(',').map(|x| x.trim().parse::<u32>().unwrap()) {
        let count = match qwe.get(&pos) {
            Some(&pos) => pos,
            None => 0,
        };
        qwe.insert(pos, count + 1);
    }

    qwe
}

fn step1(positions: HashMap<u32, u32>) -> usize {
    let max_pos = *(positions.keys().max().unwrap());
    let min_pos = *(positions.keys().min().unwrap());
    let mut best_cost = std::usize::MAX;

    for pos in min_pos..=max_pos {
        // compute fuel cost
        let cost = positions.iter().fold(0 as usize, |cost, (&pos2, &n_crabs)| {
            let diff = if pos > pos2 {pos - pos2} else {pos2 - pos};
            cost + (n_crabs * diff) as usize
        });

        if cost < best_cost {
            best_cost = cost;
        }
    }

    best_cost
}

fn step2(positions: HashMap<u32, u32>) -> usize {
    let max_pos = *(positions.keys().max().unwrap());
    let min_pos = *(positions.keys().min().unwrap());
    let mut best_cost = std::usize::MAX;

    for pos in min_pos..=max_pos {
        // compute fuel cost
        let cost = positions.iter().fold(0 as usize, |cost, (&pos2, &n_crabs)| {
            let diff = if pos > pos2 {pos - pos2} else {pos2 - pos};
            let diff2: u32 = (0..=diff).sum();
            cost + (n_crabs * diff2) as usize
        });

        if cost < best_cost {
            best_cost = cost;
        }
    }

    best_cost
}

fn main() {
    let test = "16,1,2,0,4,2,7,1,2,14";
    let test_input = parse_input(&test);
    assert_eq!(37, step1(test_input));
    let test_input = parse_input(&test);
    assert_eq!(168, step2(test_input));

    let input = include_str!("day7.input");
    let map = parse_input(&input);
    println!("{}", step1(map));
    let map = parse_input(&input);
    println!("{}", step2(map));
}