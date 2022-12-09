use std::collections::HashSet;
use std::str::FromStr;

pub struct Head {
    pub current_pos: Point,
}

impl Head {
    pub fn instruct_move(self: &mut Head, ins: Instructions) -> Vec<Point> {
        let start = self.current_pos.clone();
        let mut path: Vec<Point> = Vec::new();
        println!("========{:?}", ins);
        match ins.direction {
            Direction::Right => {
                self.current_pos.x += ins.steps;
                for x in start.x..self.current_pos.x {
                    path.push(Point {
                        x: x + 1,
                        y: self.current_pos.y,
                    });
                }
            }
            Direction::Left => {
                self.current_pos.x -= ins.steps;
                for x in (self.current_pos.x..start.x + 1).rev() {
                    path.push(Point {
                        x: x,
                        y: self.current_pos.y,
                    });
                }
            }
            Direction::Up => {
                self.current_pos.y += ins.steps;
                for y in start.y..self.current_pos.y {
                    path.push(Point {
                        x: self.current_pos.x,
                        y: y + 1,
                    });
                }
            }
            Direction::Down => {
                self.current_pos.y -= ins.steps;
                for y in (self.current_pos.y..start.y + 1).rev() {
                    path.push(Point {
                        x: self.current_pos.x,
                        y: y,
                    });
                }
            }
        };
        return path;
    }
}

pub struct Tail {
    pub current_pos: Point,
    pub visited: HashSet<Point>,
}

impl Tail {
    pub fn follow(self: &mut Tail, head_path: &Vec<Point>) {
        for head_pos in head_path {
            println!("head{:?}", head_pos);
            let diff_x = head_pos.x - self.current_pos.x;
            let diff_y = head_pos.y - self.current_pos.y;
            if diff_x.abs() > 1 {
                self.current_pos.x += diff_x / 2;
                self.current_pos.y += diff_y;
            }
            if diff_y.abs() > 1 {
                self.current_pos.x += diff_x;
                self.current_pos.y += diff_y / 2;
            }

            println!("tail{:?}\n", self.current_pos);

            self.visited.insert(self.current_pos.clone());
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Instructions {
    pub direction: Direction,
    pub steps: i32,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

impl FromStr for Instructions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        Ok(Instructions {
            direction: Direction::from_str(split.next().unwrap()).unwrap(),
            steps: split.next().unwrap().parse().unwrap(),
        })
    }
}

pub fn parse(input: &str) -> Vec<Instructions> {
    input
        .lines()
        .map(|l| Instructions::from_str(l).unwrap())
        .collect()
}
