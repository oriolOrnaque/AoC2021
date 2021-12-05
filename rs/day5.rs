use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(input: &str) -> Self {
        let (x, y) = input.split_once(',').unwrap();
        Point {
            x: x.trim().parse::<i32>().unwrap(),
            y: y.trim().parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(input: &str) -> Self {
        let (p1, p2) = input.split_once("->").unwrap();
        Line {
            p1: Point::new(p1),
            p2: Point::new(p2),
        }
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        if (self.p1.x == self.p2.x) || (self.p1.y == self.p2.y) {
            return true;
        }
        false
    }

    fn generate_points(&self) -> Vec<Point> {
        let (mut x0, mut y0, x1, y1) = (self.p1.x, self.p1.y, self.p2.x, self.p2.y);
        let mut points: Vec<Point> = Vec::new();

        // bresenham's line algorithm
        let dx = (x1 - x0).abs();
        let dy = -((y1 - y0).abs());
        let sx = if x0 < x1 {1} else {-1};
        let sy = if y0 < y1 {1} else {-1};
        let mut err = dx + dy;
        loop {
            points.push(Point{x: x0, y: y0});
            if (x0 == x1) && (y0 == y1) {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }

        points
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(|line| Line::new(line)).collect()
}

fn step1(lines: &Vec<Line>) -> usize{
    let mut map: HashMap<Point, u32> = HashMap::new();
    let lines: Vec<Line> = lines.iter().filter(|line| line.is_horizontal_or_vertical()).cloned().collect();

    for line in lines {
        for point in line.generate_points() {
            let count = match map.get(&point) {
                Some(&count) => count,
                None => 0,
            };
            map.insert(point, count + 1);
        }
    }

    map.values().filter(|&&count| count >= 2).count()
}

fn step2(lines: &Vec<Line>) -> usize{
    let mut map: HashMap<Point, u32> = HashMap::new();

    for line in lines {
        for point in line.generate_points() {
            let count = match map.get(&point) {
                Some(&count) => count,
                None => 0,
            };
            map.insert(point, count + 1);
        }
    }

    map.values().filter(|&&count| count >= 2).count()
}

fn main() {
    let test = include_str!("day5.test.input");
    let lines = parse_input(&test);
    assert_eq!(5, step1(&lines));
    assert_eq!(12, step2(&lines));

    let input = include_str!("day5.input");
    let lines = parse_input(&input);
    println!("{}", step1(&lines));
    println!("{}", step2(&lines));
}