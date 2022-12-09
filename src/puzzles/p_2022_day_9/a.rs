use super::utils::{parse, Head, Instructions, Point, Tail};

pub fn solve(input: &str) -> String {
    let instructions: Vec<Instructions> = parse(input);

    let start_point = Point { x: 0, y: 0 };
    let mut head = Head {
        current_pos: start_point.clone(),
    };
    let mut tail = Tail::new(1, &start_point);

    for ins in instructions {
        let head_path = head.instruct_move(ins);
        tail.follow(&head_path);
       // for y in (0..100).rev() {
       //     for x in 0..100 {
       //         let p = Point { x: x, y: y };
       //         if head.current_pos == p {
       //             print!("H ");
       //         } else {
       //             if tail.visited.contains(&p) {
       //                 if tail.current_pos == p {
       //                     print!("T ");
       //                 } else {
       //                     print!("# ");
       //                 }
       //             } else {
       //                 print!(". ");
       //             }
       //         }
       //     }
       //     print!("\n");
       // }
    }
    return tail.get_tail_end_visited_len().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small() {
        let input = "
R 4
U 1";
        let res = solve(&input[1..]);
        let expected_result = "4";
        assert_eq!(expected_result, res);
    }
    #[test]
    fn small_2() {
        let input = "
R 4
U 2";
        let res = solve(&input[1..]);
        let expected_result = "5";
        assert_eq!(expected_result, res);
    }
    #[test]
    fn given_example() {
        let input = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let res = solve(&input[1..]);
        let expected_result = "13";
        assert_eq!(expected_result, res);
    }
}
