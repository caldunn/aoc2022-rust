use std::cmp::Ordering;

pub fn main() -> std::io::Result<()> {
    let lines = crate::read_file("day13/input");
    let mut base: Vec<_> = lines
        .chunks(3)
        .map(|s| {
            let mut i = s.iter().take(2);
            (
                Value::parse_new(i.next().unwrap()),
                Value::parse_new(i.next().unwrap()),
            )
        })
        .collect();

    let part1: usize = base
        .iter()
        .enumerate()
        .filter(|(_, (x, y))| Ordering::Less == compare(&x, &y))
        .map(|(i, _)| i + 1)
        .sum();
    let (div1, div2) = (Value::parse_new("[[2]]"), Value::parse_new("[[6]]"));
    base.push((div1.clone(), div2.clone()));

    let mut part2 = base
        .iter()
        .flat_map(|(x, y)| vec![x, y])
        .collect::<Vec<_>>();
    part2.sort_by(|x, y| compare(x, y));

    let div1_i = part2.iter().position(|v| **v == div1).unwrap() + 1;
    let div2_i = part2.iter().position(|v| **v == div2).unwrap() + 1;

    println!("Result\nP1: {}\nP2: {}", part1, div1_i * div2_i);

    Ok(())
}

// Primary struct for nested vectors.

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Val(usize),
    Arr(Box<Vec<Value>>),
}

impl Value {
    fn parse_new(line: &str) -> Self {
        let mut builder: Vec<Vec<Value>> = vec![vec![]];
        for s in tokenise(line) {
            match s {
                "[" => builder.push(vec![]),
                "]" => {
                    let arr = builder.pop().unwrap();
                    builder.last_mut().unwrap().push(Value::Arr(Box::new(arr)))
                }
                "," => {}
                x => builder
                    .last_mut()
                    .unwrap()
                    .push(Value::Val(x.parse().unwrap())),
            };
        }
        Self::Arr(Box::new(builder.pop().unwrap()))
    }
}

fn compare(v1: &Value, v2: &Value) -> Ordering {
    match (v1, v2) {
        (Value::Val(vx), Value::Val(vy)) => vx.cmp(vy),
        (Value::Arr(ax), Value::Arr(ay)) => {
            for (v1, v2) in ax.iter().zip(ay.iter()) {
                match compare(v1, v2) {
                    Ordering::Equal => {}
                    x => return x,
                };
            }
            ax.len().cmp(&ay.len())
        }
        (Value::Val(vx), right) => compare(&Value::Arr(Box::new(vec![Value::Val(*vx)])), right),
        (left, Value::Val(vy)) => compare(left, &Value::Arr(Box::new(vec![Value::Val(*vy)]))),
    }
}

fn tokenise(s: &str) -> Vec<&str> {
    s.split_inclusive("[")
        .flat_map(|s| {
            s.split_inclusive(",")
                .flat_map(|ss| ss.split_inclusive("]"))
        })
        // My cheap workaround for digits inclusive.
        .flat_map(|s| {
            if s.len() == 1 {
                vec![s]
            } else {
                let x = s.split_at(s.len() - 1);
                vec![x.0, x.1]
            }
        })
        .collect()
}
