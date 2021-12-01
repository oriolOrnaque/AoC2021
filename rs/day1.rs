use std::io::BufRead;

fn count_positive_deltas(xs: &Vec<i32>) -> usize {
    let mut count: usize = 0;

    for i in 1..xs.len() {
        if xs[i] - xs[i-1] > 0 {
            count += 1;
        }
    }

    count
}

fn count_positive_windows(xs: &Vec<i32>) -> usize {
    let mut windows: Vec<i32> = Vec::with_capacity(xs.len() - 2);

    for i in 2..xs.len() {
        windows.push(xs[i] + xs[i-1] + xs[i-2]);
    }
    
    count_positive_deltas(&windows)
}

fn main() {
    let test = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    assert_eq!(count_positive_deltas(&test), 7);
    assert_eq!(count_positive_windows(&test), 5);

    match std::fs::File::open("day1.input") {
        Ok(f) => {
            let reader = std::io::BufReader::new(f);
            let xs = reader.lines()
                .filter(|line| line.is_ok())
                .map(|line| line.unwrap().parse::<i32>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect();
            println!("Step 1: {}", count_positive_deltas(&xs));
            println!("Step 2: {}", count_positive_windows(&xs));
        },
        _ => (),
    }
}