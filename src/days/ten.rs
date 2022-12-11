pub fn main() -> std::io::Result<()> {
    let lines = crate::read_file("day10/input");
    let mut samples = vec![];
    let (mut cycle, mut xreg) = (0, 1);
    for line in lines {
        let mut split = line.splitn(2, " ");
        match (split.next(), split.next()) {
            (Some("noop"), _) => tick(xreg, &mut cycle, &mut samples),
            (Some("addx"), Some(x)) => {
                tick(xreg, &mut cycle, &mut samples);
                tick(xreg, &mut cycle, &mut samples);
                xreg += x.parse::<i32>().unwrap()
            }
            _ => todo!(),
        }
    }
    let score = samples.iter().fold(0, |acc, s| acc + s);
    println!("{:?}", score);
    Ok(())
}
fn tick(xreg: i32, cycle: &mut i32, samples: &mut Vec<i32>) {
    *cycle += 1;
    if *cycle % 40 == 20 {
        samples.push(xreg * *cycle);
    };
    let draw_cycle = *cycle - 1;
    let symbol = if ((draw_cycle % 40) - xreg).abs() <= 1 {
        "#"
    } else {
        "."
    };
    print!("{}", symbol);
    if draw_cycle % 40 == 39 {
        println!("");
    }
}
