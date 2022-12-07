use std::collections::HashMap;

use super::utils::parse;

pub fn solve(input: &str) -> String {
    let folder_sizes: HashMap<String, u32> = parse(input);

    let sum: u32 = folder_sizes.values().filter(|v| **v <= 100000).sum();
    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_nbr() {
        let input = "
$ cd /
$ ls
100000 b.txt";
        let res = solve(&input[1..]);
        let expected_result = "100000";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn incorrect_nbr() {
        let input = "
$ cd /
$ ls
100001 b.txt";
        let res = solve(&input[1..]);
        let expected_result = "0";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn summerize() {
        let input = "
$ cd /
$ ls
dir a
1 b.txt
$ cd a
$ ls
3 r.txt";
        let res = solve(&input[1..]);
        let expected_result = "7";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn change_dir() {
        let input = "
$ cd /
$ ls
1 b.txt
$ cd a
$ ls
1 bb.txt
$ cd ..";
        let res = solve(&input[1..]);
        let expected_result = "3";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn correct_sum_dir() {
        let input = "
$ cd /
$ ls
dir a
1 b.txt
100000 c.txt
$ cd a
$ ls
3 r.txt";
        let res = solve(&input[1..]);
        let expected_result = "3";
        assert_eq!(res, expected_result);
    }
    #[test]
    fn given_example() {
        let input = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let res = solve(&input[1..]);
        let expected_result = "95437";
        assert_eq!(res, expected_result);
    }
}
