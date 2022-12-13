use super::utils::{parse, find_shortest_path_to_end};

pub fn solve(input: &str) -> String {
    let (map, start_coords) = parse(input);
    let shortest_distance = find_shortest_path_to_end(&map, start_coords);
    return shortest_distance.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small() {
        let input = "
SabcdefghijklmnopqrstuvwxyE";
        let res = solve(&input[1..]);
        let expected_result = "26";
        assert_eq!(expected_result, res);
    }
    #[test]
    fn small_2() {
        let input = "
SabcdefghijklmnopqrstuvwxyyyE";
        let res = solve(&input[1..]);
        let expected_result = "28";
        assert_eq!(expected_result, res);
    }
    #[test]
    fn given_example() {
        let input = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let res = solve(&input[1..]);
        let expected_result = "31";
        assert_eq!(expected_result, res);
    }
}
