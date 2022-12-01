fn parse_instructions(input: &str) -> Vec<Vec<i32>> {
    let mut helpers_backpacks = Vec::new();
    let mut current_helper = Vec::new();
    for l in input.lines() {
        match l.parse::<i32>() {
            Ok(nbr) => current_helper.push(nbr),
            Err(_) => {
                helpers_backpacks.push(current_helper.clone());
                current_helper = Vec::new();
            }
        }
    }
    helpers_backpacks.push(current_helper.clone());
    return helpers_backpacks;
}


pub fn solve_a(input: &str) -> i32 {
    let helpers_backpacks: Vec<Vec<i32>> = parse_instructions(input);
    let most_calories = helpers_backpacks.iter().fold(0, |highest_calories, backpack| {
        let current_calories = backpack.iter().sum();
        if current_calories > highest_calories {
            return current_calories;
        } else {
            return highest_calories;
        }
    });
    return most_calories;
}

pub fn solve_b(input: &str) -> i32 {
    solve_a(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let result = 24000;
        assert_eq!(solve_a(input), result);
    }
}
