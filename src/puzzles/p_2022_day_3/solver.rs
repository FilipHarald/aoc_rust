fn find_shared_item<'a>(compartment_one: &'a [char], compartment_two: &'a [char]) -> &'a char {
    let mut found_item: Option<&char> = None;
    for item in compartment_one.iter() {
        found_item = compartment_two.iter().find(|&x| x == item);
        if found_item != None {
            break;
        }
    }
    return found_item.unwrap();
}

const SMALL_LETTER_UNICODE_START: u16 = 96;
const BIG_LETTER_UNICODE_START: u16 = 64;
const ADDED_BIG_LETTER_VAL: u16 = 26;

fn get_priority(c: &char) -> i32 {
    let unicode_val = *c as u16;
    if unicode_val > SMALL_LETTER_UNICODE_START {
        return (unicode_val - SMALL_LETTER_UNICODE_START) as i32;
    } else {
        return (unicode_val - BIG_LETTER_UNICODE_START + ADDED_BIG_LETTER_VAL) as i32;
    }
}

pub fn solve_a(input: &str) -> String {
    let mut total_priority = 0;
    for l in input.lines() {
        let chars = l.chars().collect::<Vec<char>>();
        let (compartment_one, compartment_two) = chars.split_at(chars.len() / 2);
        let shared_item = find_shared_item(compartment_one, compartment_two);
        total_priority += get_priority(shared_item);
    }
    return total_priority.to_string();
}

fn find_shared_item_group<'a>(
    rucksack_one: &'a [char],
    group_rucksacks: Vec<Vec<char>>,
) -> &'a char {
    let mut res: &char = &'0';
    for item in rucksack_one.iter() {
        let mut found_item: bool = true;
        for rucksack in group_rucksacks.iter() {
            found_item = found_item && rucksack.contains(item);
        }
        if found_item {
            res = item;
        }
    }
    return res;
}

pub fn solve_b(input: &str) -> String {
    let mut total_priority = 0;
    let lines: Vec<&str> = input.lines().collect();
    for (index, l) in lines.iter().enumerate() {
        if (index + 1) % 3 == 0 {
            let one: Vec<char> = lines.get(index - 2).unwrap().chars().collect();
            let two: Vec<char> = lines.get(index - 1).unwrap().chars().collect();
            let three: Vec<char> = l.chars().collect();
            let shared_item = find_shared_item_group(&one, vec![two, three]);
            total_priority += get_priority(shared_item);
        }
    }
    return total_priority.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = "157";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_2() {
        let input = "aa";
        let result = "1";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_3() {
        let input = "AA";
        let result = "27";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn b_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = "70";
        assert_eq!(solve_b(input), result);
    }
    #[test]
    fn b_2() {
        let input = "aa
aa
aa
baab
bb
bb";
        let result = "3";
        assert_eq!(solve_b(input), result);
    }
}
