use super::utils::parse;

pub fn solve(input: &str) -> String {
    let clearing_pairs = parse(input);
    let overlapping_pairs = clearing_pairs.iter().fold(0, |acc, (a, b)| {
        if a.fully_contains(b) || b.fully_contains(a) {
            return acc + 1;
        }
        return acc;
    });
    return overlapping_pairs.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_1() {
        let input = "2-4,2-4";
        let res = solve(input);
        let expected_result = "1";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn simple_2() {
        let input = "2-4,1-3";
        let res = solve(input);
        let expected_result = "0";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn simple_3() {
        let input = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let res = solve(&input[1..]);
        let expected_result = "2";
        assert_eq!(res, expected_result);
    }
}
