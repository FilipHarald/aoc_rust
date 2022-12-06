use super::parser::parse;

pub fn solve(input: &str) -> String {
    let parsed_input = parse(input);
    let mut first_marker: usize = 0;
    for i in 0..parsed_input.len() {
        if i > 3 {
            let mut past_four = parsed_input.get(i-4..i).unwrap().to_vec();
            past_four.sort();
            past_four.dedup();

            if past_four.len() == 4 {
                first_marker = i;
                break;
            }
        }
    }
    return first_marker.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let res = solve(input);
        let expected_result = "7";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn simple_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let res = solve(input);
        let expected_result = "5";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn simple_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let res = solve(input);
        let expected_result = "6";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn simple_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let res = solve(input);
        let expected_result = "10";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn simple_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let res = solve(input);
        let expected_result = "11";
        assert_eq!(res, expected_result);
    }
}
