use super::parser::parse;

const START_OF_MESSAGE_MARKER: usize = 14;

pub fn solve(input: &str) -> String {
    let parsed_input = parse(input);
    let mut first_marker: usize = 0;
    for i in 0..parsed_input.len() {
        if i >= START_OF_MESSAGE_MARKER {
            let range = i-START_OF_MESSAGE_MARKER..i;
            let mut past_four = parsed_input.get(range).unwrap().to_vec();
            past_four.sort();
            past_four.dedup();

            if past_four.len() == START_OF_MESSAGE_MARKER {
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
        let expected_result = "19";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn simple_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let res = solve(input);
        let expected_result = "23";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn simple_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let res = solve(input);
        let expected_result = "23";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn simple_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let res = solve(input);
        let expected_result = "29";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn simple_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let res = solve(input);
        let expected_result = "26";
        assert_eq!(res, expected_result);
    }
}

