use std::collections::HashMap;

use super::utils::parse;

pub fn solve(input: &str) -> String {
    let folder_sizes: HashMap<String, u32> = parse(input);

    let total_disk_available: u32 = 70000000;
    let required_free: u32 = 30000000;
    let required_to_free: u32 = folder_sizes.get("/").unwrap() + required_free - total_disk_available;


    let best_folder_size: u32 = folder_sizes.values().fold(u32::max_value(), |best_size, curr_folder_size| {
        if curr_folder_size < &best_size && curr_folder_size > &required_to_free {
            return *curr_folder_size;
        }
        return best_size;
    });
    return best_folder_size.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let expected_result = "24933642";
        assert_eq!(res, expected_result);
    }
}

