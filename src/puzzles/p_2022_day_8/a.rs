use super::utils::{parse, Tree};

fn can_see_out_from_tree(t: &Tree, tree_yard: &Vec<Vec<Tree>>) -> (bool, bool) {
    let mut x_axis = t.is_visible_from_outside_x;
    let mut y_axis = t.is_visible_from_outside_y;
    if !x_axis && !y_axis {
        for x in (0..t.x).rev() {
            let current_tree = &tree_yard[t.y][x];
            if current_tree.height >= t.height {
                break;
            }
            x_axis = x_axis || current_tree.is_visible_from_outside_x;
         }
    }
    if !x_axis && !y_axis {
        for y in (0..t.y).rev() {
            let current_tree = &tree_yard[y][t.x];
            if current_tree.height >= t.height {
                break;
            }
            y_axis = y_axis || current_tree.is_visible_from_outside_y;
        }
    }
    if !x_axis && !y_axis {
        for x in t.x + 1..tree_yard[t.y].len() {
            let current_tree = &tree_yard[t.y][x];
            if current_tree.height >= t.height {
                break;
            }
            x_axis = x_axis || current_tree.is_visible_from_outside_x;
        }
    }
    if !x_axis && !y_axis {
        for y in t.y + 1..tree_yard.len() {
            let current_tree = &tree_yard[y][t.x];
            if current_tree.height >= t.height {
                break;
            }
            y_axis = y_axis || current_tree.is_visible_from_outside_y;
        }
    }

    return (x_axis, y_axis);
}

pub fn solve(input: &str) -> String {
    let mut tree_yard = parse(input);

    for y in 0..tree_yard.len() {
        for x in 0..tree_yard[y].len() {
            let is_vis: (bool, bool) = can_see_out_from_tree(&tree_yard[y][x], &tree_yard);
            tree_yard[y][x].is_visible_from_outside_x = is_vis.0;
            tree_yard[y][x].is_visible_from_outside_y = is_vis.1;
        }
    }

    let visible_trees: usize = tree_yard.iter().fold(0, |sum, row| {
        sum + row
            .iter()
            .filter(|tree| tree.is_visible_from_outside_x || tree.is_visible_from_outside_y)
            .count()
    });
    return visible_trees.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_1() {
        let input = "
111
121
111";
        let res = solve(&input[1..]);
        let expected_result = "9";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn small_2() {
        let input = "
111
101
111";
        let res = solve(&input[1..]);
        let expected_result = "8";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn small_same_height() {
        let input = "
111
111
111";
        let res = solve(&input[1..]);
        let expected_result = "8";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn bigger_neighbour() {
        let input = "
9999
1329
9999";
        let res = solve(&input[1..]);
        let expected_result = "11";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn bigger_neighbour_right() {
        let input = "
9999
9231
9999";
        let res = solve(&input[1..]);
        let expected_result = "11";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn bigger_neighbour_down() {
        let input = "
999
929
939
919";
        let res = solve(&input[1..]);
        let expected_result = "11";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn given_example() {
        let input = "
30373
25512
65332
33549
35390";
        let res = solve(&input[1..]);
        let expected_result = "21";
        assert_eq!(res, expected_result);
    }
}
