use std::collections::HashSet;
use std::str::FromStr;

pub struct Head {
    pub current_pos: Point,
}

impl Head {
    pub fn instruct_move(self: &mut Head, ins: Instructions) -> Vec<Point> {
        let start = self.current_pos.clone();
        let mut path: Vec<Point> = Vec::new();
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
                        x,
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
                        y,
                    });
                }
            }
        };
        return path;
    }
}

pub struct Tail {
    pub current_pos: Point,
    pub id: usize,
    pub tail: Option<Box<Tail>>,
    pub visited: Option<HashSet<Point>>,
}

impl Tail {
    pub fn new(tail_size: usize, start: &Point) -> Self {
        let mut current = Self {
            current_pos: start.clone(),
            id: tail_size,
            tail: None,
            visited: Some(HashSet::new()),
        };
        current.visited.as_mut().unwrap().insert(start.clone());
        for tail_id in (1..tail_size).rev() {
            let new_tail = Self {
                current_pos: start.clone(),
                id: tail_id,
                tail: Some(Box::new(current)),
                visited: None,
            };
            current = new_tail;
        }
        return current;
    }

    fn calculate_new_pos(self: &Tail, parent_pos: &Point) -> Option<Point> {
        let diff_x = parent_pos.x - self.current_pos.x;
        let diff_y = parent_pos.y - self.current_pos.y;
        if diff_x.abs() > 1 && diff_y.abs() > 1 {
            return Some(Point {
                x: self.current_pos.x + (diff_x / 2),
                y: self.current_pos.y + (diff_y / 2),
            });
        }
        if diff_x.abs() > 1 {
            return Some(Point {
                x: self.current_pos.x + (diff_x / 2),
                y: self.current_pos.y + diff_y,
            });
        };
        if diff_y.abs() > 1 {
            return Some(Point {
                x: self.current_pos.x + diff_x,
                y: self.current_pos.y + (diff_y / 2),
            });
        };
        return None;
    }

    pub fn follow(self: &mut Tail, head_path: &Vec<Point>) {
        for head_pos in head_path {
            self.adjust_tail(head_pos.clone());
        }
    }

    pub fn adjust_tail(self: &mut Tail, parent_pos: Point) {
        match self.calculate_new_pos(&parent_pos) {
            Some(new_pos) => {
                self.current_pos = new_pos;
                match &mut self.tail {
                    Some(t) => {
                        t.adjust_tail(self.current_pos.clone());
                    }
                    None => {
                        self.visited
                            .as_mut()
                            .unwrap()
                            .insert(self.current_pos.clone());
                    }
                }
            }
            None => (),
        };
    }

    pub fn get_tail_end_visited_len(self: &mut Tail) -> usize {
        match &mut self.tail {
            Some(t) => t.get_tail_end_visited_len(),
            None => self.visited.as_ref().unwrap().len().clone(),
        }
    }
    pub fn _get_print(self: &mut Tail, p: &Point) -> String {
        if self.current_pos == p.clone() {
            return format!("{} ", self.id);
        } else {
            match &mut self.tail {
                Some(t) => t._get_print(p),
                None => {
                    if self.visited.as_ref().unwrap().contains(p) {
                        return "# ".to_string();
                    } else {
                        return ". ".to_string();
                    }
                }
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
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
