fn preprocess(input: &str) -> Vec<(&str, i32)> {
    input.lines()
        .map(|line| line.split_once(' '))
        .filter(|opt| opt.is_some())
        .map(|opt| opt.unwrap())
        .map(|(instr, num)| (instr, num.parse::<i32>().unwrap()))
        .collect()
}

fn step1(input: &Vec<(&str, i32)>) -> i32 {
    let (mut x, mut y) = (0, 0);

    for (instr, num) in input {
        match *instr {
            "forward" => x += num,
            "down" => y += num,
            "up" => y -= num,
            _ => {},
        }
    }

    x * y
}

fn step2(input: &Vec<(&str, i32)>) -> i32 {
    let (mut pos_x, mut pos_y) = (0, 0);
    let mut aim_x = 0;

    for (instr, num) in input {
        match *instr {
            "down" => aim_x += num,
            "up" => aim_x -= num,
            "forward" => {
                pos_x += num;
                pos_y += aim_x * num;
            },
            _ => {},
        }
    }

    pos_x * pos_y
}

fn main() {
    let raw_test = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
    let test = preprocess(&raw_test);
    assert_eq!(step1(&test), 150);
    assert_eq!(step2(&test), 900);

    let raw_input = std::fs::read_to_string("day2.input").unwrap();
    let input = preprocess(&raw_input);

    println!("Step 1 solution: {}", step1(&input));
    println!("Step 2 solution: {}", step2(&input));
}