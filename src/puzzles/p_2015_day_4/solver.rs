use md5;

pub fn run() {
    // TODO: read input
    // TODO: solve
    // TODO: print metrics?
    println!("hello world");
}

fn solve_a(input: &str) -> i32{
    let mut counter = -1;
    let mut is_match = false;
    while !is_match {
        counter += 1;
        let mut owned_input = input.to_owned();
        owned_input.push_str(&format!("{}", counter));
        let digest = md5::compute(owned_input);
        let digest_string = format!("{:x}", digest);
        is_match = digest_string.starts_with("00000");
    }
    return counter;
}

fn solve_b(input: &str) -> i32{
    let mut counter = -1;
    let mut is_match = false;
    while !is_match {
        counter += 1;
        let mut owned_input = input.to_owned();
        owned_input.push_str(&format!("{}", counter));
        let digest = md5::compute(owned_input);
        let digest_string = format!("{:x}", digest);
        is_match = digest_string.starts_with("000000");
    }
    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_simple_1() {
        let input = "abcdef";
        let result = 609043;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_simple_2() {
        let input = "pqrstuv";
        let result = 1048970;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_proper() {
        let input = "iwrupvqb";
        let result = 346386;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn b_proper() {
        let input = "iwrupvqb";
        let result = 9958218;
        assert_eq!(solve_b(input), result);
    }
}
