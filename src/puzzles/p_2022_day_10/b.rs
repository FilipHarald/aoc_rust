use super::utils::{parse, Instructions};

pub fn solve(input: &str) -> String {
    let instructions = parse(input);
    return do_instructions(instructions);
}

fn do_instructions(instructions: Vec<Instructions>) -> String {
    let mut cycle_counter: i32 = 0;
    let mut reg_val: i32 = 1;
    let mut crt: String = String::new();

    for ins in instructions.iter() {
        let unified_ins = match ins {
            Instructions::Noop { cycles } => (*cycles, 0),
            Instructions::Addx { cycles, add } => (*cycles, *add),
        };
        for _i in 0..unified_ins.0 {
            cycle_counter += 1;
            let sprite_is_on_pixel = (reg_val + 1 - cycle_counter).abs() <= 1;
            if sprite_is_on_pixel {
                crt.push('#');
            } else {
                crt.push('.');
            }
            if cycle_counter > 0 && cycle_counter % 40 == 0 {
                cycle_counter = 0;
                crt.push('\n');
            }
        }
        reg_val += unified_ins.1;
    }
    return crt;
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let expected_result = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(expected_result[1..], res);
    }
}
