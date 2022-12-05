pub fn solve_a(input: &str) -> String {
    let mut tot = 0;
    for line in input.lines() {
        let measurements: Vec<i32> = line
            .trim()
            .split("x")
            .map(|val| val.parse().unwrap())
            .collect();
        let side_1 = measurements[0] * measurements[1];
        let side_2 = measurements[1] * measurements[2];
        let side_3 = measurements[0] * measurements[2];
        let &min_val = vec![side_1, side_2, side_3].iter().min().unwrap();
        tot += 2 * side_1 + 2 * side_2 + 2 * side_3 + min_val;

    }
    return tot.to_string();
}

pub fn solve_b(input: &str) -> String {
    let mut tot = 0;
    for line in input.lines() {
        let mut measurements: Vec<i32> = line
            .trim()
            .split("x")
            .map(|val| val.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        measurements.sort();
        tot += 2 * measurements[0] + 2 * measurements[1] + (measurements[0] * measurements[1] * measurements[2]);
    }
    return tot.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_simple_1() {
        let input = "2x3x4";
        let result = "58";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_2() {
        let input = "1x1x10";
        let result = "43";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_3() {
        let input = "1x1x10
2x3x4";
        let result = "101";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn b_simple_1() {
        let input = "2x3x4";
        let result = "34";
        assert_eq!(solve_b(input), result);
    }
    #[test]
    fn b_simple_2() {
        let input = "1x1x10";
        let result = "14";
        assert_eq!(solve_b(input), result);
    }
}
