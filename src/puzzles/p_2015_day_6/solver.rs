enum Command {
    Toggle,
    TurnOn,
    TurnOff
}

fn parse_instructions(line: &str) -> (Command, Vec<usize>, Vec<usize>) {
    let mut c: Command = Command::Toggle;
    let mut from_point_string = "";

    let words: Vec<&str> = line.split(" ").collect();
    if line.starts_with("toggle") {
        c = Command::Toggle;
        from_point_string = words[1];
    }
    if line.starts_with("turn on") {
        c = Command::TurnOn;
        from_point_string = words[2];
    }
    if line.starts_with("turn off") {
        c = Command::TurnOff;
        from_point_string = words[2];
    }

    let from_point = from_point_string
        .split(",")
        .map(
            |x| x.parse::<usize>().unwrap()
            )
        .collect();
    let to_point = words[words.len() - 1]
        .split(",")
        .map(
            |x| x.parse::<usize>().unwrap()
            )
        .collect();

    return (
        c,
        from_point,
        to_point
        )
}

fn do_instructions_a(lights: &mut Vec<Vec<bool>>, instructions: (Command, Vec<usize>, Vec<usize>)) {
    for x in instructions.1[0]..=instructions.2[0] {
        for y in instructions.1[1]..=instructions.2[1] {
            match instructions.0 {
                Command::Toggle  => lights[x][y] = !lights[x][y],
                Command::TurnOn  => lights[x][y] = true,
                Command::TurnOff => lights[x][y] = false,
            }
        }
    }
}

pub fn solve_a(input: &str) -> String {
    let mut lights = vec![vec![false; 1000]; 1000];
    for l in input.lines() {
        let instructions = parse_instructions(l);
        do_instructions_a(&mut lights, instructions);
    }
    return lights.into_iter().fold(0, |matrix_acc, light_row| {
        matrix_acc + light_row.into_iter().fold(0, |row_acc, is_on| {
            return if is_on { row_acc + 1 } else { row_acc }
        })
    }).to_string();
}

fn do_instructions_b(lights: &mut Vec<Vec<i32>>, instructions: (Command, Vec<usize>, Vec<usize>)) {
    for x in instructions.1[0]..=instructions.2[0] {
        for y in instructions.1[1]..=instructions.2[1] {
            match instructions.0 {
                Command::Toggle  => lights[x][y] = lights[x][y] + 2,
                Command::TurnOn  => lights[x][y] = lights[x][y] + 1,
                Command::TurnOff => lights[x][y] = if lights[x][y] == 0 { 0 } else { lights[x][y] - 1 },
            }
        }
    }
}

pub fn solve_b(input: &str) -> String {
    let mut lights = vec![vec![0; 1000]; 1000];
    for l in input.lines() {
        let instructions = parse_instructions(l);
        do_instructions_b(&mut lights, instructions);
    }
    return lights.into_iter().fold(0, |acc, light_row| {
        let sum: i32 = light_row.iter().sum();
        return acc + sum;
    }).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_1() {
        let input = "turn on 0,0 through 999,999";
        let result = "1000000";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_2() {
        let input = "turn on 0,0 through 999,0
turn off 0,0 through 499,0";
        let result = "500";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_3() {
        let input = "turn on 499,499 through 500,500";
        let result = "4";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_4() {
        let input = "turn on 499,499 through 500,500
toggle 499,499 through 500,499";
        let result = "2";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn b_1() {
        let input = "turn on 499,499 through 500,500";
        let result = "4";
        assert_eq!(solve_b(input), result);
    }
    #[test]
    fn b_2() {
        let input = "turn on 499,499 through 500,500
toggle 499,499 through 500,499";
        let result = "8";
        assert_eq!(solve_b(input), result);
    }
    #[test]
    fn b_3() {
        let input = "turn on 0,0 through 0,2
turn off 0,0 through 0,1
turn off 0,0 through 0,1";
        let result = "1";
        assert_eq!(solve_b(input), result);
    }
}
