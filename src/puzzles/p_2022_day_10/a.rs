use super::utils::{parse, Instructions};

pub fn solve(input: &str) -> String {
    let instructions = parse(input);
    let res = do_instructions(instructions);
    return res.1.to_string();
}

fn do_instructions(instructions: Vec<Instructions>) -> (i32, i32) {
    let mut cycle_counter: i32 = 0;
    let mut reg_val: i32 = 1;
    let mut interesting_signal_strength = 0;
    for ins in instructions.iter() {
        let unified_ins = match ins {
            Instructions::Noop { cycles } => (*cycles, 0),
            Instructions::Addx { cycles, add } => (*cycles, *add),
        };
        for _i in 0..unified_ins.0 {
            cycle_counter += 1;
            if (cycle_counter - 20) % 40 == 0 || cycle_counter == 20 {
                interesting_signal_strength += cycle_counter * reg_val;
            }
        }
        reg_val += unified_ins.1;
    }
    return (reg_val, interesting_signal_strength);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small() {
        let input = "
noop
addx 3
addx -5";
        let instructions = parse(&input[1..]);
        let res = do_instructions(instructions);
        let expected_result = -1;
        assert_eq!(expected_result, res.0);
    }
    #[test]
    fn given_example() {
        let input = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let res = solve(&input[1..]);
        let expected_result = "13140";
        assert_eq!(expected_result, res);
    }
}
