pub fn main() -> std::io::Result<()> {
    let lines = crate::read_file("day11/input");
    let mut monkeys_p1 = parse_monkeys(&lines);
    let mut monkeys_p2 = parse_monkeys(&lines);
    let lcm_mod = monkeys_p1.iter().fold(1, |acc, m| acc * m.divisor);
    let res_1 = run_simulation(&mut monkeys_p1, 0..20, None);
    let res_2 = run_simulation(&mut monkeys_p2, 0..10_000, Some(lcm_mod));
    crate::print_results(res_1, res_2);
    Ok(())
}

fn run_simulation(
    monkeys: &mut Vec<Monkey>,
    range: std::ops::Range<usize>,
    control: Option<u64>,
) -> usize {
    for _ in range {
        for i in 0..monkeys.len() {
            let throws = monkeys.get_mut(i).unwrap().play_with_items(control);
            throws.into_iter().for_each(|t| {
                monkeys
                    .get_mut(t.monkey_idx as usize)
                    .unwrap()
                    .items
                    .push(t.item)
            });
        }
    }
    monkeys.sort_by(|m1, m2| m1.inspections.cmp(&m2.inspections));
    monkeys
        .iter()
        .rev()
        .take(2)
        .fold(1, |acc, m| acc * m.inspections)
}

fn parse_monkeys(lines: &[String]) -> Vec<Monkey> {
    lines
        .chunks(7)
        .map(|monkey_raw| {
            let items: Vec<u64> = monkey_raw[1]
                .split_at(18)
                .1
                .split(",")
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect();
            let operator =
                Operator::from_char(monkey_raw[2].split_at(23).1.chars().next().unwrap());
            let constant = monkey_raw[2]
                .split_at(25)
                .1
                .parse::<u64>()
                .map(|i| Constant::Const(i))
                .unwrap_or(Constant::Old);
            let divisor = monkey_raw[3].split_at(21).1.parse().unwrap();
            let on_true = monkey_raw[4].split_at(29).1.parse().unwrap();
            let on_false = monkey_raw[5].split_at(30).1.parse().unwrap();
            Monkey {
                items,
                operator,
                constant,
                divisor,
                on_false,
                on_true,
                inspections: 0,
            }
        })
        .collect()
}
#[derive(Debug)]
enum Operator {
    Add,
    Mult,
}
#[derive(Debug)]
enum Constant {
    Const(u64),
    Old,
}
impl Operator {
    fn from_char(c: char) -> Self {
        match c {
            '+' => Operator::Add,
            '*' => Operator::Mult,
            _ => panic!(),
        }
    }
}
#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operator: Operator,
    constant: Constant,
    divisor: u64,
    on_true: usize,
    on_false: usize,
    inspections: usize,
}

impl Monkey {
    fn play_with_items(&mut self, lcm_mod: Option<u64>) -> Vec<Throw> {
        let throws = self
            .items
            .iter()
            .map(|i| {
                self.inspections += 1;
                let item_presquash = match (&self.operator, &self.constant) {
                    (Operator::Add, Constant::Const(x)) => i + x,
                    (Operator::Add, Constant::Old) => i + i,
                    (Operator::Mult, Constant::Const(x)) => i * x,
                    (Operator::Mult, Constant::Old) => i * i,
                };
                let item = lcm_mod
                    .map(|m| item_presquash % m)
                    .unwrap_or_else(|| item_presquash / 3);

                let monkey_idx = if item % self.divisor == 0 {
                    self.on_true
                } else {
                    self.on_false
                };
                Throw { item, monkey_idx }
            })
            .collect();
        self.items.clear(); // Unable able to use into iter as it moves items to function from
                            // self.
        throws
    }
}

struct Throw {
    item: u64,
    monkey_idx: usize,
}
