fn parse_input(input: &str) -> [usize; 9] {
    let mut counters = [0 as usize; 9];
    for timer in input.split(',').map(|x| x.trim().parse::<usize>().unwrap()) {
        counters[timer + 1] += 1;
    }
    counters
}

fn simulate(counters: &mut [usize], days: usize) -> usize {
    for _ in 0..days {
        counters[7] += counters[0];
        counters.rotate_left(1);
        //println!("{:?}", counters);
    }
    counters[7] += counters[0];
    counters.iter().sum()
}

fn main() {
    let test = "3,4,3,1,2";
    let mut test_counters = parse_input(&test);
    assert_eq!(5934, simulate(&mut test_counters, 80));

    let input = include_str!("day6.input");
    let mut counters = parse_input(&input);
    println!("{}", simulate(&mut counters, 80));

    let mut counters = parse_input(&input);
    println!("{}", simulate(&mut counters, 256));
}