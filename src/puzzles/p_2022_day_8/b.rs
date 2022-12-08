use super::utils::{parse, Tree};

fn get_scenic_score(t: &Tree, tree_yard: &Vec<Vec<Tree>>) -> usize {
    let mut dist_left = 0;
    let mut dist_up = 0;
    let mut dist_right = 0;
    let mut dist_down = 0;

    for x in (0..t.x).rev() {
        let current_tree = &tree_yard[t.y][x];
        dist_left += 1;
        if current_tree.height >= t.height {
            break;
        }
    }
    for y in (0..t.y).rev() {
        let current_tree = &tree_yard[y][t.x];
        dist_up += 1;
        if current_tree.height >= t.height {
            break;
        }
    }
    for x in t.x + 1..tree_yard[t.y].len() {
        let current_tree = &tree_yard[t.y][x];
        dist_right += 1;
        if current_tree.height >= t.height {
            break;
        }
    }
    for y in t.y + 1..tree_yard.len() {
        let current_tree = &tree_yard[y][t.x];
        dist_down += 1;
        if current_tree.height >= t.height {
            break;
        }
    }
    return dist_left * dist_up * dist_right * dist_down;
}

pub fn solve(input: &str) -> String {
    let tree_yard = parse(input);

    let mut highest_scenic_score: u32 = 0;
    for y in 0..tree_yard.len() {
        for x in 0..tree_yard[y].len() {
            let scenic_score = get_scenic_score(&tree_yard[y][x], &tree_yard) as u32;
            highest_scenic_score = if scenic_score > highest_scenic_score {
                scenic_score
            } else {
                highest_scenic_score
            };
        }
    }

    return highest_scenic_score.to_string();
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
        let expected_result = "1";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn small_2() {
        let input = "
111
101
111";
        let res = solve(&input[1..]);
        let expected_result = "1";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn small_same_height() {
        let input = "
111
111
111";
        let res = solve(&input[1..]);
        let expected_result = "1";
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
        let expected_result = "8";
        assert_eq!(res, expected_result);
    }
}
