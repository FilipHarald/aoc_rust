use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
pub enum Operator {
    Addition,
    Multiplication,
}

#[derive(Debug)]
pub struct Operation {
    pub op_1: String,
    pub op_2: String,
    pub operator: Operator,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // e.g:"  Operation: new = old * 19"
        let members: Vec<&str> = s.split("new = ").collect::<Vec<&str>>()[1]
            .split(' ')
            .collect();
        let op_1 = members[0].to_string();
        let op_2 = members[2].to_string();
        let operator = match members[1] {
            "+" => Operator::Addition,
            _ => Operator::Multiplication,
        };
        return Ok(Self {
            op_1,
            op_2,
            operator,
        });
    }
}

#[derive(Debug)]
pub struct Monkey {
    pub items: VecDeque<u128>,
    pub inscpected_counter: u128,
    pub operation: Operation,
    pub div_test: u128,
    pub div_outcome: (u128, u128),
}

impl Monkey {
    fn get_operation_member(op: &String, item: u128) -> u128 {
        match op.parse::<u128>() {
            Ok(nbr) => nbr,
            Err(_) => item,
        }
    }

    pub fn do_operation(self: &Monkey, item: u128, common_denominator: u128) -> u128 {
        let op_1 = Self::get_operation_member(&self.operation.op_1, item);
        let op_2 = Self::get_operation_member(&self.operation.op_2, item);
        //println!("{op_1} {:?} {op_2}", self.operation.operator);
        match self.operation.operator {
            Operator::Addition => op_1 + op_2,
            Operator::Multiplication => {
                let one = op_1 % common_denominator;
                let two = op_2 % common_denominator;
                return one * two;
            },
        }
    }

    pub fn do_div_test(self: &Monkey, item: u128) -> u128 {
        if item % self.div_test == 0 {
            self.div_outcome.0
        } else {
            self.div_outcome.1
        }
    }
}

fn parse_items(item_string: &str) -> VecDeque<u128> {
    // e.g:"   Starting items: 79, 98"
    item_string
        .split(&[' ', ','])
        .fold(VecDeque::new(), |mut items, s| {
            match s.parse::<u128>() {
                Ok(nbr) => {
                    items.push_back(nbr);
                }
                Err(_) => (),
            }
            return items;
        })
}
fn get_number_at_end(s: &str) -> u128 {
    s.split(' ').last().unwrap().parse().unwrap()
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next();
        let items = parse_items(lines.next().unwrap());
        let operation = Operation::from_str(lines.next().unwrap()).unwrap();
        let div_test = get_number_at_end(lines.next().unwrap());
        let outcome_true = get_number_at_end(lines.next().unwrap());
        let outcome_false = get_number_at_end(lines.next().unwrap());
        return Ok(Self {
            items,
            inscpected_counter: 0,
            operation,
            div_test,
            div_outcome: (outcome_true, outcome_false),
        });
    }
}

pub fn parse(input: &str) -> Vec<Monkey> {
    let mut monkey_strings = input.split("Monkey ");
    monkey_strings.next();
    let monkeys: Vec<Monkey> = monkey_strings
        .map(|s| Monkey::from_str(s).unwrap())
        .collect();
    return monkeys;
}
