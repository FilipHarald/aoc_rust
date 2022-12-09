use super::utils::{parse, Head, Instructions, Point, Tail};

pub fn solve(input: &str) -> String {
    let instructions: Vec<Instructions> = parse(input);

    let start_point = Point { x: 0, y: 0 };
    let mut head = Head {
        current_pos: start_point.clone(),
    };
    let mut tail = Tail::new(9, &start_point);

    for ins in instructions {
        let head_path = head.instruct_move(ins);
        tail.follow(&head_path);
//        for y in (-5..10).rev() {
//            for x in -5..10 {
//                let p = Point { x, y };
//                if head.current_pos == p {
//                    print!("H ");
//                } else {
//                    let to_print = tail.get_print(&p);
//                    print!("{to_print}");
//                }
//            }
//            print!("\n");
//        }
    }
    return tail.get_tail_end_visited_len().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small() {
        let input = "
U 12";
        let res = solve(&input[1..]);
        let expected_result = "4";
        assert_eq!(expected_result, res);
    }
    #[test]
    fn given_example_1() {
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
        let expected_result = "1";
        assert_eq!(expected_result, res);
    }
    #[test]
    fn small_2() {
        let input = "
R 5
U 8";
        let res = solve(&input[1..]);
        let expected_result = "1";
        assert_eq!(expected_result, res);
    }
    #[test]
    fn given_example_2() {
        let input = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let res = solve(&input[1..]);
        let expected_result = "36";
        assert_eq!(expected_result, res);
    }
}

