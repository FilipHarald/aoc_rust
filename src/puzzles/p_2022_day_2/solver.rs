fn calculate_points(opponent: char, me: char) -> i32 {
    match me {
        'X' => {
            1 + match opponent {
                'A' => 3,
                'B' => 0,
                'C' => 6,
                _ => 0,
            }
        }
        'Y' => {
            2 + match opponent {
                'A' => 6,
                'B' => 3,
                'C' => 0,
                _ => 0,
            }
        }
        'Z' => {
            3 + match opponent {
                'A' => 0,
                'B' => 6,
                'C' => 3,
                _ => 0,
            }
        }
        _ => 0,
    }
}

pub fn solve_a(input: &str) -> i32 {
    let mut score = 0;
    for l in input.lines() {
        let chars: Vec<char> = l.chars().collect();
        score += calculate_points(chars[0], chars[2]);
    }
    return score;
}

fn decide_move(opponent: char, me: char) -> char {
    match me {
        'X' => match opponent {
            'A' => 'Z',
            'B' => 'X',
            'C' => 'Y',
            _ => 'Ö',
        },
        'Y' => match opponent {
            'A' => 'X',
            'B' => 'Y',
            'C' => 'Z',
            _ => 'Ö',
        },
        'Z' => match opponent {
            'A' => 'Y',
            'B' => 'Z',
            'C' => 'X',
            _ => 'Ö',
        },
        _ => 'Ö',
    }
}

pub fn solve_b(input: &str) -> i32 {
    let mut score = 0;
    for l in input.lines() {
        let chars: Vec<char> = l.chars().collect();
        let my_move = decide_move(chars[0], chars[2]);
        score += calculate_points(chars[0], my_move);
    }
    return score;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_1() {
        let input = "A Y
B X
C Z";
        let result = 15;
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn b_1() {
        let input = "A Y
B X
C Z";
        let result = 12;
        assert_eq!(solve_b(input), result);
    }
}
