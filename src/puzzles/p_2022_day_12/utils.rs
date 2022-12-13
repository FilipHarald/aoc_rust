use std::collections::{HashMap, HashSet, VecDeque};

pub fn get_valid_neighbors<'a>(
    node: (usize, usize),
    node_val: &'a char,
    all_nodes: &'a Vec<Vec<char>>,
) -> Vec<((usize, usize), &'a char)> {
    let mut neighbors = Vec::new();

    if node.0 > 0 {
        let neighbor_x = node.0 -1;
        let neighbor_y = node.1;
        match all_nodes.get(neighbor_y).and_then(|axis| axis.get(neighbor_x)) {
            Some(neighbor_char) if is_valid_neighbor(neighbor_char, node_val) => {
                neighbors.push(((neighbor_x, neighbor_y), neighbor_char));
            }
            _ => (),
        }
    }

    if node.1 > 0 {
        let neighbor_x = node.0;
        let neighbor_y = node.1 - 1;
        match all_nodes.get(neighbor_y).and_then(|axis| axis.get(neighbor_x)) {
            Some(neighbor_char) if is_valid_neighbor(neighbor_char, node_val) => {
                neighbors.push(((neighbor_x, neighbor_y), neighbor_char));
            }
            _ => (),
        }
    }

    if node.0 + 1 < all_nodes.get(0).unwrap().len() {
        let neighbor_x = node.0 + 1;
        let neighbor_y = node.1;
        match all_nodes.get(neighbor_y).and_then(|axis| axis.get(neighbor_x)) {
            Some(neighbor_char) if is_valid_neighbor(neighbor_char, node_val) => {
                neighbors.push(((neighbor_x, neighbor_y), neighbor_char));
            }
            _ => (),
        }
    }
    if node.1 + 1 < all_nodes.len() {
        let neighbor_x = node.0;
        let neighbor_y = node.1 + 1;
        match all_nodes.get(neighbor_y).and_then(|axis| axis.get(neighbor_x)) {
            Some(neighbor_char) if is_valid_neighbor(neighbor_char, node_val) => {
                neighbors.push(((neighbor_x, neighbor_y), neighbor_char));
            }
            _ => (),
        }
    }
    return neighbors;
}

fn is_valid_neighbor(neighbor: &char, current: &char) -> bool {
    let n_val = match neighbor {
        'S' => 'a' as u16,
        'E' => 'z' as u16,
        neighbor => *neighbor as u16,
    };

    let curr_val = match current {
        'S' => 'a' as u16,
        'E' => 'z' as u16,
        current => *current as u16,
    };
    return n_val <= curr_val + 1;
}

pub fn find_shortest_path_to_end(map: &Vec<Vec<char>>, start_coords: (usize, usize)) -> usize {
    let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();

    let mut shortest_distance = usize::MAX;

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
    shortest_distance
}

pub fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut start_x = 0;
    let mut start_y = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        matrix.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {

                start_x = x;
                start_y = y;
                matrix[y].push('a');
            } else { 

                matrix[y].push(c);
            }
        }
    }
    return (matrix, (start_x, start_y));
}
