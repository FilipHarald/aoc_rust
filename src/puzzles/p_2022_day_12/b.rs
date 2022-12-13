use super::utils::{find_shortest_path_to_end, parse};

pub fn solve(input: &str) -> String {
    let (map, _start_coords) = parse(input);
    let max_x = map.get(0).unwrap().len();
    let max_y = map.len();
    let mut shortest_distance = usize::MAX;
    for x in 0..max_x {
        for y in 0..max_y {
            let is_on_edge = x == 0 || y == 0 || x == max_x - 1 || y == max_y;
            if is_on_edge && map.get(y).unwrap().get(x).unwrap() == &'a' {
                let dist = find_shortest_path_to_end(&map, (x, y));
                if dist < shortest_distance {
                    shortest_distance = dist;
                }
            }
        }
    }
    return shortest_distance.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_example() {
        let input = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let res = solve(&input[1..]);
        let expected_result = "29";
        assert_eq!(expected_result, res);
    }
}

