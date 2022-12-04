pub fn main() -> std::io::Result<()> {
    let lines = crate::read_file("day4/input");

    let pairs: Vec<_> = lines.iter().map(|s| Pair::new(s)).collect();
    let contains: Vec<_> = pairs.iter().filter(|p| p.fully_contained()).collect();
    let overlaps: Vec<_> = pairs.iter().filter(|p| p.any_overlap()).collect();
    println!("RESULTS\nPairs contained: {}\nPairs overlapped: {}", contains.len(), overlaps.len());
    Ok(())
}

#[derive(Debug)]
struct Pair {
    a: SimpleRange,
    b: SimpleRange,
}
impl Pair {
    fn new(s: &str) -> Self {
        let split = s.split(",").collect::<Vec<_>>();
        Self {
            a: SimpleRange::from_string(split.get(0).expect("first pair missing")),
            b: SimpleRange::from_string(split.get(1).expect("first pair missing")),
        }
    }

    fn fully_contained(&self) -> bool {
        self.a.low <= self.b.low && self.a.high >= self.b.high
            || self.b.low <= self.a.low && self.b.high >= self.a.high
    }
    fn any_overlap(&self) -> bool {
        // lazy ranges -- more readable tho
        let a = self.a.low..=self.a.high;
        let b = self.b.low..=self.b.high;
        a.contains(&self.b.low)
            || a.contains(&self.b.high)
            || b.contains(&self.a.low)
            || b.contains(&self.a.high)
    }
}
#[derive(Debug)]
struct SimpleRange {
    low: usize,
    high: usize,
}
impl SimpleRange {
    fn from_string(s: &str) -> Self {
        let split = s.split("-").collect::<Vec<_>>();
        Self {
            low: split.get(0).expect("low missing").parse().unwrap(),
            high: split.get(1).expect("high missing").parse().unwrap(),
        }
    }
}
