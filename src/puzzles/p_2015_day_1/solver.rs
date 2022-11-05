pub fn solve_a(input: &str) -> i32 {
    let mut floor_counter = 0;
    // TODO: redo with iterator
    for c in input.chars() {
        if c == '(' {
            floor_counter = floor_counter + 1;
        } else {
            floor_counter = floor_counter - 1;
        }
    }
    return floor_counter;
}

pub fn solve_b(input: &str) -> i32 {
    const BASEMENT_FLOOR: i32 = -1;
    let mut floor_counter = 0;
    let mut basement_index_counter: i32 = -1;
    for (index, c) in input.chars().enumerate() {
        if c == '(' {
            floor_counter = floor_counter + 1;
        } else {
            floor_counter = floor_counter - 1;
            if floor_counter == BASEMENT_FLOOR {
                basement_index_counter = index as i32 + 1;
                break;
            }
        }
    };
    return basement_index_counter;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_simple_1() {
        let input = "(())";
        let result = 0;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_2() {
        let input = "()()";
        let result = 0;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_3() {
        let input = "(((";
        let result = 3;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_4() {
        let input = "(()(()(";
        let result = 3;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_5() {
        let input = "))(((((";
        let result = 3;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_6() {
        let input = "())";
        let result = -1;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_7() {
        let input = "))(";
        let result = -1;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_8() {
        let input = ")))";
        let result = -3;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_9() {
        let input = ")())())";
        let result = -3;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn b_simple_1() {
        let input = ")";
        let result = 1;
        assert_eq!(solve_b(input), result);
    }
    #[test]
    fn b_simple_2() {
        let input = "()())";
        let result = 5;
        assert_eq!(solve_b(input), result);
    }
}
