use super::utils::parse;

const ROUNDS: usize = 10000;

pub fn solve(input: &str) -> String {
    let mut monkeys = parse(input);
    for _round in 0..ROUNDS {
        let common_denominator = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.div_test);
        //println!("== After round {_round} == ({common_denominator})");

        for m_i in 0..monkeys.len() {
            while monkeys[m_i].items.len() > 0 {
                let worry_prev = monkeys[m_i].items.pop_front().unwrap();
                monkeys[m_i].inscpected_counter += 1;
                let worry_new = monkeys[m_i].do_operation(worry_prev, common_denominator);
                let next_monkey: usize = monkeys[m_i].do_div_test(worry_new).try_into().unwrap();

                //println!("Monkey {m_i}, worry_prev {worry_prev}, worry_new {worry_new}, next_monkey {next_monkey}");
                //let worry_after_thinking = worry_new % common_denominator;
                monkeys[next_monkey].items.push_back(worry_new);
            }
            if _round % 20 == 0 {
                //println!(
                    //"Monkey {m_i} inspected items {} times.",
                    //monkeys[m_i].inscpected_counter
                //);
            }
        }
    }
    //println!("{:#?}", monkeys);
    monkeys.sort_by(|a, b| b.inscpected_counter.cmp(&a.inscpected_counter));
    return monkeys[0..2]
        .iter()
        .fold(1, |acc, monkey| acc * monkey.inscpected_counter)
        .to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small() {
        let input = "
Monkey 0:
  Starting items: 2, 3
  Operation: new = old * 10
  Test: divisible by 2
    If true: throw to monkey 1
    If false: throw to monkey 2

Monkey 1:
  Starting items: 1
  Operation: new = old + 1
  Test: divisible by 3
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 1
  Operation: new = old * old
  Test: divisible by 4
    If true: throw to monkey 0
    If false: throw to monkey 1

";
        let res = solve(&input[1..]);
        let expected_result = "1066573335";
        assert_eq!(expected_result, res);
    }
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
        let expected_result = "2713310158";
        assert_eq!(expected_result, res);
    }
}
