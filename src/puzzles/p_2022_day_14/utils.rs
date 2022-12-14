use std::{cmp, collections::HashSet, str::FromStr};
//pub enum PixelType {
//    Air,
//    Rock,
//    Sand
//}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x_y: Vec<i32> = s.split(",").map(|n| n.parse().unwrap()).collect();
        return Ok(Point {
            x: x_y[0],
            y: x_y[1],
        });
    }
}

pub fn to_cave_map(
    rock: HashSet<Point>,
    (min_x, min_y, max_x, max_y): (i32, i32, i32, i32),
) -> String {
    let mut s = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if rock.contains(&Point { x, y }) {
                s += "#";
            } else if x == 500 && y == 0 {
                s += "+";
            } else {
                s += ".";
            }
        }
        s += "\n";
    }
    return s;
}

pub fn parse(input: &str) -> (HashSet<Point>, (i32, i32, i32, i32)) {
    let mut min_x = i32::MAX;
    let mut min_y = 0;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut rock = HashSet::new();
    for line in input.lines() {
        let rock_nodes: Vec<Point> = line
            .split(" -> ")
            .map(|string_point| Point::from_str(string_point).unwrap())
            .collect();
        for i in 0..rock_nodes.len() - 1 {
            let diff_x = rock_nodes[i].x - rock_nodes[i + 1].x;
            let diff_y = rock_nodes[i].y - rock_nodes[i + 1].y;
            let mut step_x = 0;
            let mut step_y = 0;
            let edge_len = if diff_x.abs() > diff_y.abs() {
                step_x = if diff_x < 0 { 1 } else { -1 };
                diff_x.abs() + 1
            } else {
                step_y = if diff_y < 0 { 1 } else { -1 };
                diff_y.abs() + 1
            };
            for offset in 0..edge_len {
                let x = rock_nodes[i].x + offset * step_x;
                let y = rock_nodes[i].y + offset * step_y;
                min_x = cmp::min(min_x, x);
                min_y = cmp::min(min_y, y);
                max_x = cmp::max(max_x, x);
                max_y = cmp::max(max_y, y);
                rock.insert(Point { x, y });
            }
        }
    }
    return (rock, (min_x, min_y, max_x, max_y));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let (rock, limits) = parse(&input[1..]);
        assert_eq!(20, rock.len());
        assert_eq!(494, limits.0, "min x incorrect");
        assert_eq!(0, limits.1, "min y incorrect");
        assert_eq!(503, limits.2, "max x incorrect");
        assert_eq!(9, limits.3, "max y incorrect");
    }
    #[test]
    fn test_visualize() {
        let input = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let (rock, limits) = parse(&input[1..]);
        let res = to_cave_map(rock, limits);
        let expected_result = "
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.
";
        assert_eq!(&expected_result[1..], res);
    }
}
