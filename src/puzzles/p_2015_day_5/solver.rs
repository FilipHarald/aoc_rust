fn is_nice_vowel(c: char) -> bool {
    return "aeiou".contains(c);
}

fn check_for_double_letter(two_letters: &str) -> bool {
    return two_letters.chars().nth(0) == two_letters.chars().nth(1);
}

const NAUGHTY_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn check_for_naughty_string(two_letters: &str) -> bool {
    return NAUGHTY_STRINGS.contains(&two_letters);
}

fn check_string_is_nice(string: &str) -> bool {
    let mut at_least_three_nice_vowels = false;
    let mut at_least_one_double_letter = false;
    let mut has_naughty_string = false;

    let mut nice_vowel_counter = 0;
    for (i, c) in string.chars().enumerate() {
        if !at_least_three_nice_vowels {
            if is_nice_vowel(c) {
                nice_vowel_counter += 1;
            }
            at_least_three_nice_vowels = nice_vowel_counter >= 3;
        }
        if i > 0 {
            let two_letters = &string[i-1..i+1];
            at_least_one_double_letter = at_least_one_double_letter || check_for_double_letter(two_letters);
            has_naughty_string = has_naughty_string || check_for_naughty_string(two_letters);
        }
    }
    return at_least_three_nice_vowels && at_least_one_double_letter && !has_naughty_string;
}

pub fn solve_a(input: &str) -> i32 {
    let mut counter = 0;
    for l in input.lines() {
        if check_string_is_nice(l) {
            counter += 1;
        }
    }
    return  counter;
}

pub fn solve_b(input: &str) -> i32 {
    println!("{}", input);
    return 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_simple_1() {
        let input = "ugknbfddgicrmopn";
        let result = true;
        assert_eq!(check_string_is_nice(input), result);
    }
    #[test]
    fn a_simple_2() {
        let input = "aaa";
        let result = true;
        assert_eq!(check_string_is_nice(input), result);
    }
    #[test]
    fn a_simple_3() {
        let input = "jchzalrnumimnmhp";
        let result = false; // no double letter
        assert_eq!(check_string_is_nice(input), result);
    }
    #[test]
    fn a_simple_4() {
        let input = "haegwjzuvuyypxyu";
        let result = false; // has xy
        assert_eq!(check_string_is_nice(input), result);
    }
    #[test]
    fn a_simple_5() {
        let input = "dvszwmarrgswjxmb";
        let result = false; // only 1 vowel
        assert_eq!(check_string_is_nice(input), result);
    }
}
