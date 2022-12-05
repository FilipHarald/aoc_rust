use std::collections::HashMap;

fn parse(lines: &str) -> HashMap<&str, Vec<&str>> {
    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();
    for l in lines.lines() {
        let cave_edge: Vec<&str> = l.split("-").collect();
        caves
            .entry(cave_edge[0])
            .or_insert(Vec::new())
            .push(cave_edge[1]);
        caves
            .entry(cave_edge[1])
            .or_insert(Vec::new())
            .push(cave_edge[0]);
    }
    return caves;
}

fn check_is_valid_cave_connection(
    cave_id: &str,
    path_to_this_cave: &Vec<&str>,
    has_visited_small_cave_twice: bool,
) -> (bool, bool) {
    if cave_id.to_uppercase() == cave_id {
        return (true, has_visited_small_cave_twice);
    }
    if cave_id == "start" {
        return (false, has_visited_small_cave_twice);
    }
    let cave_id_has_been_visited = path_to_this_cave.contains(&cave_id);
    if has_visited_small_cave_twice {
        return (!cave_id_has_been_visited, has_visited_small_cave_twice);
    }
    return (true, cave_id_has_been_visited);
}

fn recursive_find_possible_end_paths<'a>(
    connections: &HashMap<&str, Vec<&'a str>>,
    path_to_this_cave: Vec<&'a str>,
    has_already_visited_small_cave_twice: bool,
) -> Vec<Vec<&'a str>> {
    let current_cave_id = path_to_this_cave.last().unwrap();
    connections.get(current_cave_id).unwrap().iter().fold(
        Vec::new(),
        |mut possible_end_paths, connection_cave_id| {
            let (is_valid_cave_connection, has_visited_small_cave_twice) =
                check_is_valid_cave_connection(
                    connection_cave_id,
                    &path_to_this_cave,
                    has_already_visited_small_cave_twice,
                );
            if is_valid_cave_connection {
                let mut path = path_to_this_cave.clone();
                path.push(connection_cave_id);
                if connection_cave_id.to_owned() == "end" {
                    possible_end_paths.push(path)
                } else {
                    let mut more_possible_end_paths = recursive_find_possible_end_paths(
                        &connections,
                        path,
                        has_visited_small_cave_twice,
                    );
                    possible_end_paths.append(&mut more_possible_end_paths);
                }
            }
            return possible_end_paths;
        },
    )
}

pub fn solve_a(input: &str) -> String {
    let connections = parse(input);
    let possible_end_paths =
        recursive_find_possible_end_paths(&connections, vec!["start"], true);
    return possible_end_paths.len().to_string();
}

pub fn solve_b(input: &str) -> String {
    let connections = parse(input);
    let possible_end_paths =
        recursive_find_possible_end_paths(&connections, vec!["start"], false);
    return possible_end_paths.len().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_1() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let result = "10";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_2() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let result = "19";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_3() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let result = "226";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn b_1() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let result = "36";
        assert_eq!(solve_b(input), result);
    }
    #[test]
    fn b_2() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let result = "103";
        assert_eq!(solve_b(input), result);
    }
    #[test]
    fn b_3() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let result = "3509";
        assert_eq!(solve_b(input), result);
    }
}
