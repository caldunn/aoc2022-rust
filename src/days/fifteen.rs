use std::collections::HashMap;
use std::ops::Range;

use regex::Regex;

type PointPairs<'a> = &'a [(Point, Point)];

pub fn main() -> std::io::Result<()> {
    let lines = crate::read_file("day15/input");
    let point_pairs = extract_pairs(lines.as_slice());
    let part_1 = part_1(point_pairs.as_slice(), 2_000_000);
    let part_2 = part_2(point_pairs.as_slice(), 4_000_000);

    crate::print_results(part_1, part_2);

    Ok(())
}

fn part_1(point_pairs: PointPairs, row: isize) -> isize {
    let map = collect_into_ranges_row(point_pairs, row);
    map.iter().fold(0, |acc, r| acc + (r.end - r.start))
}

fn part_2(point_pairs: PointPairs, max: isize) -> isize {
    let res: Vec<_> = (0..=max)
        .flat_map(|i| {
            let ranges = collect_into_ranges_row(point_pairs, i);
            match ranges.len() {
                2 if ranges[1].start - ranges[0].end == 2 => {
                    Some((ranges.first().unwrap().end + 1, i))
                }
                _ => None,
            }
        })
        .filter(|(x, _)| *x >= 0 && *x <= max)
        .collect();

    let first = res.first().unwrap();
    first.0 * 4_000_000 + first.1
}

fn collect_into_ranges_row(points: &[(Point, Point)], y: isize) -> Vec<Range<isize>> {
    let mut ranges = points
        .iter()
        .flat_map(|(sensor, beacon)| {
            let m_distance = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();

            let row_dist = (sensor.y - y).abs();

            let x_dist = m_distance - row_dist;
            if x_dist >= 0 {
                Some(sensor.x - x_dist..sensor.x + x_dist)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| (r.start, r.end));

    let mut merged: Vec<Range<isize>> = vec![];
    for range in &ranges {
        match merged.last_mut() {
            Some(last) if range.start <= last.end => last.end = last.end.max(range.end),
            _ => merged.push(range.clone()),
        }
    }
    merged
}

fn extract_pairs(lines: &[String]) -> Vec<(Point, Point)> {
    let re = Regex::new(r"[0-9]+").unwrap();
    lines
        .iter()
        .map(|line| {
            let v: Vec<_> = re
                .captures_iter(&line)
                .filter_map(|p| p.get(0).map(|pp| pp.as_str().parse::<isize>().unwrap()))
                .collect::<Vec<isize>>();

            (Point::new(v[0], v[1]), Point::new(v[2], v[3]))
        })
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let lines = crate::read_file("day15/test_part1_input");
        let pairs = extract_pairs(lines.as_slice());
        let count_at_y10 = super::part_1(pairs.as_slice(), 10);

        assert_eq!(count_at_y10, 26);
    }

    #[test]
    fn part_2() {
        let lines = crate::read_file("day15/test_part1_input");
        let pairs = extract_pairs(lines.as_slice());
        let part_2 = super::part_2(pairs.as_slice(), 20);

        assert_eq!(part_2, 56000012)
    }
}

type CaveMap = HashMap<Point, Block>;

#[derive(PartialEq, Eq)]
enum Block {
    Beacon,
    Signal,
    ConfirmedNo,
}

#[allow(dead_code)]
fn get_count_at_brute(y: isize, map: &CaveMap) -> usize {
    map.iter()
        .filter(|(point, block)| {
            point.y == y && **block != Block::Signal && **block != Block::Beacon
        })
        .fold(0, |acc, _| acc + 1)
}

#[allow(dead_code)]
fn generate_map_brute(point_pairs: &[(Point, Point)], target: isize) -> CaveMap {
    let mut map = HashMap::new();
    for (signal, beacon) in point_pairs.iter() {
        map.insert(signal.clone(), Block::Signal);
        map.insert(beacon.clone(), Block::Beacon);
        let m_distance = (signal.x - beacon.x).abs() + (signal.y - beacon.y).abs();
        for y in 0..=m_distance {
            if !(signal.y + y == target || signal.y - y == target) {
                continue;
            }
            println!("signal: {:?}", signal);
            let x_count = m_distance - y;
            for x in 0..=x_count {
                let points = vec![
                    Point::new(signal.x + x, signal.y + y),
                    Point::new(signal.x + x, signal.y - y),
                    Point::new(signal.x - x, signal.y + y),
                    Point::new(signal.x - x, signal.y - y),
                ];
                for point in points {
                    match map.get(&point) {
                        None => {
                            map.insert(point, Block::ConfirmedNo);
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    map
}
