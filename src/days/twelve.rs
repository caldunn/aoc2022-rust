use pathfinding::prelude::bfs;
pub fn main() -> std::io::Result<()> {
    let lines = crate::read_file("day12/input");
    let (map, start, end) = {
        let mut map: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
        let (start, end) = get_start_end(&map);
        map[start.i as usize][start.j as usize] = 'a';
        map[end.i as usize][end.j as usize] = 'z';
        (map, start, end)
    };
    let part_1 = bfs(
        &start,
        |p| p.successor(&map),
        |p| p.i == end.i && p.j == end.j,
    )
    .unwrap()
    .len()
        - 1;

    // Part 2 - I am going to be lazy and just bfs every point
    let part_2 = map
        .iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter().enumerate().filter_map(move |(j, c)| {
                (*c == 'a').then(|| Point {
                    level: 'a',
                    i: i as isize,
                    j: j as isize,
                })
            })
        })
        .filter_map(|p| {
            bfs(
                &p,
                |pi| pi.successor(&map),
                |pi| pi.i == end.i && pi.j == end.j,
            )
        })
        .min_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len()
        - 1;

    crate::print_results(part_1, part_2);
    Ok(())
}

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    level: char,
    i: isize,
    j: isize,
}
impl Point {
    fn from_tuple(level: char, t: (isize, isize)) -> Self {
        Self {
            level,
            i: t.0,
            j: t.1,
        }
    }
    fn successor(&self, map: &Vec<Vec<char>>) -> Vec<Point> {
        let x = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        x.iter()
            .map(|t| (self.i + t.0, self.j + t.1))
            .map(|p| {
                map.get(p.0 as usize)
                    .map(|row| (row.get(p.1 as usize), (p.0, p.1)))
            }) // Ehh
            .filter_map(|p| Some((p?.0?, p?.1)))
            .map(|p| Point {
                level: *p.0,
                i: p.1 .0,
                j: p.1 .1,
            })
            .filter(|p| p.level as isize - self.level as isize <= 1)
            .collect()
    }
}
fn get_start_end(map: &Vec<Vec<char>>) -> (Point, Point) {
    let mut start_end = map.iter().enumerate().flat_map(|v| {
        v.1.iter()
            .enumerate()
            .filter(|(_, c)| **c == 'S' || **c == 'E')
            .map(move |p| (p.1, (v.0 as isize, p.0 as isize)))
    });

    fn double_tuple(start: (isize, isize), end: (isize, isize)) -> (Point, Point) {
        (Point::from_tuple('a', start), Point::from_tuple('z', end))
    }
    match (start_end.next(), start_end.next()) {
        (Some(('S', start)), Some(('E', end))) => double_tuple(start, end),
        (Some(('E', end)), Some(('S', start))) => double_tuple(start, end),
        _ => panic!(),
    }
}
