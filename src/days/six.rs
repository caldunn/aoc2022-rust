pub fn main() -> std::io::Result<()> {
    let datagram = &crate::read_file("day6/input")[0];
    let part1 = uniqueness(datagram, 4).unwrap();
    let part2 = uniqueness(datagram, 14).unwrap();
    println!("RESULTS\nP1: {}\nP2: {}", part1, part2);
    Ok(())
}

fn uniqueness(datagram: &String, u_len: usize) -> Option<String> {
    let mut prev: Vec<char> = datagram.chars().take(u_len).collect();
    for (i, c) in datagram.chars().skip(u_len).enumerate() {
        prev[i % u_len] = c;

        let uniques = prev
            .iter()
            .collect::<std::collections::hash_set::HashSet<_>>()
            .len();
        if uniques == u_len {
            return Some(format!("{:?} -> {}", prev, i + u_len + 1));
        }
    }
    None
}
