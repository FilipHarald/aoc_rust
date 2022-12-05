use std::collections::HashMap;

pub fn solve_a(input: &str) -> String {
    let mut vistited_houses = HashMap::<Vec<i32>, i32>::new();
    let mut x = 0;
    let mut y = 0;
    *vistited_houses.entry(vec![x, y]).or_insert(0) += 1;
    for c in input.chars() {
        match c {
            '^' => x += 1,
            'v' => x -= 1,
            '>' => y += 1,
            '<' => y -= 1,
            _ => println!("Invalid char: '{}'", c),
        }
        *vistited_houses.entry(vec![x, y]).or_insert(0) += 1;
    }
    return vistited_houses.len().to_string();
}

fn move_and_deliver(x: &mut i32, y: &mut i32, c: char, vistited_houses: &mut HashMap<Vec<i32>, i32>) {
        match c {
            // D: * I'm not sure yet why dereferencing helps here.
            '^' => *x += 1,
            'v' => *x -= 1,
            '>' => *y += 1,
            '<' => *y -= 1,
            _ => println!("Invalid char: '{}'", c),
        }
        *vistited_houses.entry(vec![*x, *y]).or_insert(0) += 1;
}

pub fn solve_b(input: &str) -> String {
    let mut vistited_houses = HashMap::<Vec<i32>, i32>::new();
    let mut santa_x = 0; // A: mutable here because we are going to change this var.
    let mut santa_y = 0;
    let mut robot_x  = 0;
    let mut robot_y = 0;
    // B: & here because we want to make sure that it is the reference we are passing on and not
    // the value.
    // C: mut here because we want the reference we pass on to be mutable.
    vistited_houses.insert(vec![0, 0], 1);
    for (index, c) in input.chars().enumerate() {
        if index % 2 == 0 {
            move_and_deliver(&mut santa_x, &mut santa_y, c, &mut vistited_houses);
        } else {
            move_and_deliver(&mut robot_x, &mut robot_y, c, &mut vistited_houses);
        }
    }
    return vistited_houses.len().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_simple_1() {
        let input = ">";
        let result = "2";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_2() {
        let input = "^>v<";
        let result = "4";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_3() {
        let input = "^v^v^v^v^v";
        let result = "2";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn b_simple_1() {
        let input = "^v";
        let result = "3";
        assert_eq!(solve_b(input), result);
    }
    #[test]
    fn b_simple_2() {
        let input = "^>v<";
        let result = "3";
        assert_eq!(solve_b(input), result);
    }
    #[test]
    fn b_simple_3() {
        let input = "^v^v^v^v^v";
        let result = "11";
        assert_eq!(solve_b(input), result);
    }
}
