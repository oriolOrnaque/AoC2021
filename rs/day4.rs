#[derive(Debug)]
struct Board {
    board: Vec<i32>,
    marked: Vec<bool>,
    sum: i32,
    won_at_round_n: usize,
}

impl Board {
    fn new(input: &str) -> Self {
        let board: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let board_len = board.len();
        let sum = board.iter().fold(0, |acc, x| acc + x);
        Board {
            board: board,
            marked: vec![false; board_len],
            sum: sum,
            won_at_round_n: 0,
        }
    }

    fn mark(&mut self, n: i32) {
        if self.won_at_round_n == 0 {
            for (idx, num) in self.board.iter().enumerate() {
                if n == *num {
                    self.marked[idx] = true;
                    self.sum -= n;
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        for i in 0..5 {
            if self.marked[5 * i + i] {
                let mut has_column = true;
                for j in 0..5 {
                    if !self.marked[5 * j + i] {
                        has_column = false;
                    }
                }

                let mut has_row = true;
                for j in 0..5 {
                    if !self.marked[5 * i + j] {
                        has_row = false;
                    }
                }

                if has_column | has_row {
                    return true;
                }
            }
        }
        false
    }
}

fn parse_input(input: &str) -> Option<(Vec<i32>, Vec<Board>)> {
    match input.split_once('\n') {
        Some((queue, boards)) => {
            let queue: Vec<i32> = queue.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            let boards: Vec<Board> = boards.split("\n\n").map(|board| Board::new(board)).collect();
            Some((queue, boards))
        },
        None => {None},
    }
}

fn step1(queue: &Vec<i32>, boards: &mut Vec<Board>) -> i32 {
    for num in queue {
        for board in &mut *boards {
            board.mark(*num);
            if board.has_bingo() {
                return board.sum * num;
            }
        }
    }
    0
}

fn step2(queue: &Vec<i32>, boards: &mut Vec<Board>) -> i32 {
    for (idx, num) in queue.iter().enumerate() {
        for board in &mut *boards {
            board.mark(*num);
            if board.has_bingo() && board.won_at_round_n == 0 {
                board.won_at_round_n = idx;
            }
        }
    }
    match boards.iter().max_by_key(|board: &&Board| board.won_at_round_n) {
        Some(board) => board.sum * queue[board.won_at_round_n],
        None => 0,
    }
}

fn main() {
    println!("Hello world");

    let test = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

    match parse_input(&test) {
        Some((queue, mut boards)) => assert_eq!(step1(&queue, &mut boards), 4512),
        _ => {},
    }
    match parse_input(&test) {
        Some((queue, mut boards)) => assert_eq!(step2(&queue, &mut boards), 1924),
        _ => {},
    }

    let input = include_str!("day4.input");
    match parse_input(&input) {
        Some((queue, mut boards)) => println!("{}", step1(&queue, &mut boards)),
        _ => {},
    }
    match parse_input(&input) {
        Some((queue, mut boards)) => println!("{}", step2(&queue, &mut boards)),
        _ => {},
    }
}