use super::utils::parse;

const ROUNDS: usize = 20;

pub fn solve(input: &str) -> String {
    let mut monkeys = parse(input);
    for _round in 0..ROUNDS {
        for m_i in 0..monkeys.len() {
            while monkeys[m_i].items.len() > 0 {
                println!("Monkey {m_i}:");

                let worry_prev = monkeys[m_i].items.pop_front().unwrap();
                println!("Monkey inspects an item with a worry level of Monkey {worry_prev}:");
                monkeys[m_i].inscpected_counter += 1;
                let worry_new = monkeys[m_i].do_operation(worry_prev);
                println!("operation to {worry_new}");
                let worry_after_cooldown = worry_new / 3;
                println!("after cooldown {worry_after_cooldown}");
                let next_monkey: usize = monkeys[m_i]
                    .do_div_test(worry_after_cooldown)
                    .try_into()
                    .unwrap();
                monkeys[next_monkey].items.push_back(worry_after_cooldown);
            }
        }
    }
    println!("{:#?}", monkeys);
    monkeys.sort_by(|a, b| b.inscpected_counter.cmp(&a.inscpected_counter));
    return monkeys[0..2].iter().fold(1, |acc, monkey| acc * monkey.inscpected_counter).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_example() {
        let input = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1

";
        let res = solve(&input[1..]);
        let expected_result = "10605";
        assert_eq!(expected_result, res);
    }
}
