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

fn add_and_check_equal_pair<'a>(all_pairs: &mut Vec<&'a str>, pair: &'a str) -> bool {
    // TODO: this func can be smaller and more readable?
    let equal_pair_index = all_pairs.iter().position(|&r| r == pair);
    let mut is_equal_without_overlap = false;
    match equal_pair_index {
        Some(i) => {
            is_equal_without_overlap = i < all_pairs.len() - 1;
        },
        _ => {},
    }
    all_pairs.push(pair.clone());
    return is_equal_without_overlap;
}

fn check_for_one_repetition_with_letter_in_between(before_previous_letter: char, current_letter: char) -> bool {
    return before_previous_letter == current_letter;
}

fn check_string_is_nice_b(string: &str) -> bool {
    let mut all_letter_pairs = Vec::new();

    let mut at_least_two_equal_pairs = false;
    let mut at_least_one_repetition_with_letter_in_between = false;

    for (i, c) in string.chars().enumerate() {
        if i > 0 {
            at_least_two_equal_pairs = at_least_two_equal_pairs  || add_and_check_equal_pair(&mut all_letter_pairs, &string[i-1..i+1]);
        }
        if i > 1 {
            at_least_one_repetition_with_letter_in_between = at_least_one_repetition_with_letter_in_between || check_for_one_repetition_with_letter_in_between(
                string.chars().nth(i-2).unwrap(),
                c
            );
        }
    }
    return at_least_one_repetition_with_letter_in_between && at_least_two_equal_pairs;
}

pub fn solve_b(input: &str) -> i32 {
    let mut counter = 0;
    for l in input.lines() {
        if check_string_is_nice_b(l) {
            counter += 1;
        }
    }
    return  counter;
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
