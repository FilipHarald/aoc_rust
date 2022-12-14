pub fn solve(input: &str) -> String {
input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_example() {
        let input = "
29";
        let res = solve(&input[1..]);
        let expected_result = "29";
        assert_eq!(expected_result, res);
    }
}

