use std::collections::{HashMap, HashSet, VecDeque};

use super::utils::{get_valid_neighbors, parse};

pub fn solve(input: &str) -> String {
    let (map, start_coords) = parse(input);
    let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();

    let mut shortest_distance = 0;

    queue.push_back((start_coords, 0));
    visited.insert(start_coords);

    while queue.len() > 0 {
        let (curr_coords, dist) = queue.pop_front().unwrap();
        let current_val = map.get(curr_coords.1).unwrap().get(curr_coords.0).unwrap();
        if current_val == &'E' {
            shortest_distance = dist;
            break;
        }

        let neighbors = get_valid_neighbors(curr_coords, current_val, &map);

        for neighbor in neighbors {
            if !visited.contains(&neighbor.0) {
                previous.insert(neighbor.0, curr_coords);
                queue.push_back((neighbor.0, dist + 1));
                visited.insert(neighbor.0);
            }
        }
    }

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
