struct MoveInstructions {
    nbr_to_move: usize, // TODO: move?!
    from: usize,
    to: usize,
}

fn add_crates(crate_stacks: &mut Vec<Vec<char>>, line: &str) {
    let mut char_iterator = line.chars();
    char_iterator.next();
    for (i, c) in char_iterator.step_by(4).enumerate() {
        if c.is_alphabetic() {
            crate_stacks.get_mut(i).unwrap().push(c);
        }
    }
}

fn parse_instructions(l: &str) -> MoveInstructions {
    let split: Vec<&str> = l.split(" ").collect();
    return MoveInstructions {
        nbr_to_move: split[1].parse().unwrap(),
        from: split[3].parse::<usize>().unwrap() - 1,
        to: split[5].parse::<usize>().unwrap() - 1,
    };
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<MoveInstructions>) {
    let mut crate_stacks: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<MoveInstructions> = Vec::new();

    let mut lines = input.lines().peekable();

    let nbr_of_stacks = (lines.peek().unwrap().len() + 1) / 4;
    for _i in 0..nbr_of_stacks {
        crate_stacks.push(Vec::new());
    }

    while !lines.peek().unwrap().contains(" 1") {
        add_crates(&mut crate_stacks, lines.next().unwrap());
    }
    for stack in crate_stacks.iter_mut() {
        stack.reverse();
    }
    lines.next(); // crate stacks indecies
    lines.next(); // input type separator

    for l in lines {
        instructions.push(parse_instructions(l));
    }

    return (crate_stacks, instructions);
}

fn do_instructions_9000(crate_stacks: &mut Vec<Vec<char>>, instructions: Vec<MoveInstructions>) {
    for ins in instructions {
        for _i in 0..ins.nbr_to_move {
            let c = crate_stacks[ins.from].pop().unwrap();
            crate_stacks[ins.to].push(c);
        }
    }
}

fn get_top_crates(crate_stacks: Vec<Vec<char>>) -> String {
    let mut res: String = String::new();
    for stack in crate_stacks.iter() {
        res.push(*(stack.last().unwrap_or(&' '))); // TODO: this doesn't look pretty
    }
    return res;
}

pub fn solve_a(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);
    do_instructions_9000(&mut stacks, instructions);
    return get_top_crates(stacks);
}

fn do_instructions_9001(crate_stacks: &mut Vec<Vec<char>>, instructions: Vec<MoveInstructions>) {
    for ins in instructions {
        let range = crate_stacks[ins.from].len() - ins.nbr_to_move..;
        let from: &mut Vec<char> = &mut crate_stacks[ins.from];
        let mut crates_to_move: Vec<char> = from.drain(range).collect();
        crate_stacks[ins.to].append(&mut crates_to_move);
    }
}

pub fn solve_b(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);
    do_instructions_9001(&mut stacks, instructions);
    return get_top_crates(stacks);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn top_crates() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1"; // NOTE: not used for test
        let (stacks, _instructions) = parse(&input[1..]);
        let res = get_top_crates(stacks);
        let expected_result = "NDP";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn a_1() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1";
        let result = "DCP";
        assert_eq!(solve_a(&input[1..]), result);
    }
    #[test]
    fn a_2() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3";
        let result = " CZ";
        assert_eq!(solve_a(&input[1..]), result);
    }
    #[test]
    fn a_3() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1";
        let result = "M Z";
        assert_eq!(solve_a(&input[1..]), result);
    }
    #[test]
    fn a_4() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = "CMZ";
        assert_eq!(solve_a(&input[1..]), result);
    }
    #[test]
    fn b_1() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1";
        let result = "DCP";
        assert_eq!(solve_b(&input[1..]), result);
    }
    #[test]
    fn b_2() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3";
        let result = " CD";
        assert_eq!(solve_b(&input[1..]), result);
    }
    #[test]
    fn b_3() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1";
        let result = "C D";
        assert_eq!(solve_b(&input[1..]), result);
    }
    #[test]
    fn a_b() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = "MCD";
        assert_eq!(solve_b(&input[1..]), result);
    }
}
