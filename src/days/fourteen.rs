use std::collections::HashMap;

pub fn main() -> std::io::Result<()> {
    let lines = crate::read_file("day14/input");

    let mut map = into_matrix(&lines);
    let void = map.iter().max_by_key(|(p, _)| p.y).unwrap().0.y;

    poor_sand(&mut map, Some(void));
    let part1 = map
        .iter()
        .filter(|(_, b)| **b == Block::Sand)
        .collect::<Vec<_>>()
        .len();

    // Part 2
    // Add the floor (arbitrarily)
    for i in 200..1000 {
        map.insert(Point::new(i, void + 2), Block::Rock);
    }
    poor_sand(&mut map, None);
    let part2 = map
        .iter()
        .filter(|(_, b)| **b == Block::Sand)
        .collect::<Vec<_>>()
        .len();

    println!("{}: {}", part1, part2);

    Ok(())
}

fn poor_sand(map: &mut HashMap<Point, Block>, void: Option<isize>) {
    let mut end = false;
    loop {
        let mut sand_pos = Point::new(500, 0);
        loop {
            if sand_pos.y >= void.unwrap_or(isize::MAX)
                || map.get(&sand_pos).unwrap_or(&Block::Space) == &Block::Sand
            {
                end = true;
                break;
            } else if traversable(&Point::new(sand_pos.x, sand_pos.y + 1), &map) {
                sand_pos = Point::new(sand_pos.x, sand_pos.y + 1);
            } else if traversable(&Point::new(sand_pos.x - 1, sand_pos.y + 1), &map) {
                sand_pos = Point::new(sand_pos.x - 1, sand_pos.y + 1);
            } else if traversable(&Point::new(sand_pos.x + 1, sand_pos.y + 1), &map) {
                sand_pos = Point::new(sand_pos.x + 1, sand_pos.y + 1);
            } else {
                map.insert(sand_pos.clone(), Block::Sand);
                break;
            }
        }

        if end {
            break;
        }
    }
}

fn traversable(point: &Point, map: &HashMap<Point, Block>) -> bool {
    match map.get(point) {
        Some(Block::Rock) => false,
        Some(Block::Sand) => false,
        _ => true,
    }
}

fn into_matrix(lines: &Vec<String>) -> HashMap<Point, Block> {
    let paths: Vec<_> = lines
        .iter()
        .map(|s| {
            s.split("->")
                .map(|ss| {
                    let mut trimmed = ss.trim().split(",");
                    Point::new(
                        trimmed.next().unwrap().parse().unwrap(),
                        trimmed.next().unwrap().parse().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let mut map: HashMap<Point, Block> = HashMap::new();
    for path in paths {
        for i in 1..path.len() {
            let (p1, p2) = (&path[i - 1], &path[i]);
            let (vector, times) = match (p2.x - p1.x, p2.y - p1.y) {
                (x, 0) if x > 0 => ((1, 0), x),
                (x, 0) if x < 0 => ((-1, 0), x.abs()),
                (0, y) if y > 0 => ((0, 1), y),
                (0, y) if y < 0 => ((0, -1), y.abs()),
                _ => unreachable!("oh wht"),
            };
            let mut adder = p1.clone();
            for _ in 0..=times {
                map.insert(adder.clone(), Block::Rock);
                adder = Point::new(adder.x + vector.0, adder.y + vector.1);
            }
        }
    }
    map.insert(Point::new(500, 0), Block::Spawn);
    map
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Block {
    Sand,
    Rock,
    Space,
    Spawn,
}
