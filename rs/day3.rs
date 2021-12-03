fn step1(input: &str) -> i32 {
    let (gamma_rate, epsilon_rate) = get_rates(&input);

    gamma_rate * epsilon_rate
}

fn step2(input: &str) -> i32 {
    let oxygen_generator_rating = i32::from_str_radix(get_oxygen_generator_rating(&input.lines().collect(), 0)[0], 2).unwrap();
    let co2_scrubber_rating = i32::from_str_radix(get_co2_scrubber_rating(&input.lines().collect(), 0)[0], 2).unwrap();

    println!("{} {}", oxygen_generator_rating, co2_scrubber_rating);

    oxygen_generator_rating * co2_scrubber_rating
}

fn most_common_least_bit(input: &Vec<&str>, idx: usize) -> (char, char) {
    let mut counter: i32 = 0;

    for line in input {
        match line.chars().nth(idx) {
            Some('1') => counter += 1,
            Some('0') => counter -= 1,
            _ => {},
        }
    }
    if counter < 0 {
        return ('0', '1')
    } else if counter > 0 {
        return ('1', '0')
    }
    ('1', '0')
}

fn get_oxygen_generator_rating<'a>(input: &Vec<&'a str>, idx: usize) -> Vec<&'a str> {
    let (bit, _) = most_common_least_bit(&input, idx);
    //println!("{}", bit);
    let output: Vec<&str> = input.iter()
        .filter(|line| line.chars().nth(idx) == Some(bit))
        .cloned()
        .collect();
    //println!("{:?}", output);
    if output.len() > 1 {
        return get_oxygen_generator_rating(&output, idx + 1);
    }
    output
}

fn get_co2_scrubber_rating<'a>(input: &Vec<&'a str>, idx: usize) -> Vec<&'a str> {
    let (_, bit) = most_common_least_bit(&input, idx);
    let output: Vec<&str> = input.iter()
        .filter(|line| line.chars().nth(idx) == Some(bit))
        .cloned()
        .collect();
    if output.len() > 1 {
        return get_co2_scrubber_rating(&output, idx + 1);
    }
    output
}

fn get_rates(input: &str) -> (i32, i32) {
    match input.find('\n') {
        Some(line_length) => {
            let mut gamma_rate = String::with_capacity(line_length);
            let mut epsilon_rate = String::with_capacity(line_length);
            for idx in 0..line_length {
                let (common, least) = most_common_least_bit(&input.lines().collect(), idx);
                gamma_rate.push(common);
                epsilon_rate.push(least);
            }
            //println!("{} {}", gamma_rate, epsilon_rate);
            (
                i32::from_str_radix(&gamma_rate, 2).unwrap(),
                i32::from_str_radix(&epsilon_rate, 2).unwrap()
            )
        },
        None => (0, 0),
    }
}

fn main() {
    let test = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
    assert_eq!(get_rates(&test), (22, 9));
    assert_eq!(step1(&test), 198);
    assert_eq!(step2(&test), 230);

    let input = include_str!("day3.input");
    println!("{}", step1(&input));
    println!("{}", step2(&input));
}