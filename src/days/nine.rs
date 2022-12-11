use std::collections::HashSet;

type Point = (i32, i32);
pub fn main() -> std::io::Result<()> {
    let lines = crate::read_file("day9/input");
    let part1 = simulate_rope(&lines, 2);
    let part2 = simulate_rope(&lines, 10);
    crate::print_results(part1, part2);
    Ok(())
}
fn simulate_rope(lines: &Vec<String>, rope_length: usize) -> usize {
    let mut path = HashSet::<Point>::from([(0, 0)]);
    let mut rope = vec![(0, 0); rope_length];
    for line in lines {
        parse_line(&line).into_iter().for_each(|d| {
            rope[0] = (rope[0].0 + d.0, rope[0].1 + d.1);
            for i in 1..rope.len() {
                let mut tail = rope[i];
                let catch_up = bounds_check(rope[i - 1], tail);
                tail = (tail.0 + catch_up.0, tail.1 + catch_up.1);
                rope[i] = tail;
            }
            path.insert(*rope.last().unwrap());
        });
    }
    path.len()
}
fn bounds_check(head: Point, tail: Point) -> Point {
    let (dx, dy) = (head.0 - (tail.0), head.1 - (tail.1));
    if dx.abs() == 2 || dy.abs() == 2 {
        (dx.signum(), dy.signum())
    } else {
        (0, 0)
    }
}

fn parse_line(s: &str) -> Movement {
    let direction = match s.chars().next().unwrap() {
        'L' => (-1, 0),
        'R' => (1, 0),
        'U' => (0, 1),
        'D' => (0, -1),
        _ => panic!(),
    };

    let magnitude = s
        .chars()
        .skip(2)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    Movement {
        magnitude,
        direction,
    }
}

struct Movement {
    magnitude: usize,
    direction: Point,
}
impl Iterator for Movement {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.magnitude == 0 {
            None
        } else {
            self.magnitude -= 1;
            Some(self.direction)
        }
    }
}
