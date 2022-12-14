#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum AxisName {
    X,
    Y,
}

struct Fold {
    along_axis: AxisName,
    value: i32,
}

struct ParsedInput {
    dots: Vec<Point>,
    folds: Vec<Fold>,
}

fn parse(input: &str) -> ParsedInput {
    let mut dots = Vec::new();
    let mut folds = Vec::new();
    for l in input.lines() {
        match l.chars().next() {
            Some(c) if c.is_digit(10) => {
                let point_vals: Vec<&str> = l.split(",").collect();
                dots.push(Point {
                    x: point_vals[0].parse().unwrap(),
                    y: point_vals[1].parse().unwrap(),
                })
            }
            Some(_) => {
                let fold_vals: Vec<&str> = l
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .get(2)
                    .unwrap()
                    .split("=")
                    .collect();
                let along_axis = if fold_vals[0] == "x" {
                    AxisName::X
                } else {
                    AxisName::Y
                };
                folds.push(Fold {
                    along_axis,
                    value: fold_vals[1].parse().unwrap(),
                });
            }
            None => continue,
        }
    }
    return ParsedInput { dots, folds };
}

fn do_folds(parsed_input: ParsedInput) -> Vec<Point> {
    let mut new_dots = parsed_input.dots.clone();

    for f in parsed_input.folds {
        for dot_index in 0..new_dots.len() {
            let old_axis_val;
            let new_dot = match f.along_axis {
                AxisName::X => {
                    old_axis_val = new_dots[dot_index].x;
                    Point {
                        x: f.value * 2 - old_axis_val,
                        y: new_dots[dot_index].y,
                    }
                }
                AxisName::Y => {
                    old_axis_val = new_dots[dot_index].y;
                    Point {
                        x: new_dots[dot_index].x,
                        y: f.value * 2 - old_axis_val,
                    }
                }
            };
            if old_axis_val > f.value && !new_dots.contains(&new_dot) {
                new_dots.push(new_dot);
            }
        }
        new_dots = new_dots
            .iter()
            .filter(|d| match &f.along_axis {
                AxisName::X => d.x < f.value,
                AxisName::Y => d.y < f.value,
            })
            .cloned()
            .collect();
    }
    return new_dots;
}

pub fn solve_a(input: &str) -> String {
    let parsed_input = parse(input);
    return do_folds(parsed_input).len().to_string();
}

fn print_dots(dots: &Vec<Point>) {
    let highest = dots
        .iter()
        .fold(Point { x: 0, y: 0 }, |mut highest_vals, d| {
            if highest_vals.x < d.x {
                highest_vals.x = d.x;
            }
            if highest_vals.y < d.y {
                highest_vals.y = d.y;
            }
            return highest_vals;
        });
    for y in 0..highest.y + 1 {
        for x in 0..highest.x + 1 {
            if dots.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

pub fn solve_b(input: &str) -> String {
    let parsed_input = parse(input);
    let new_dots = do_folds(parsed_input);
    print_dots(&new_dots);
    return new_dots.len().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_1() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7";
        let result = "17";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn b_1() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let result = "16";
        assert_eq!(solve_b(input), result);
    }
}
