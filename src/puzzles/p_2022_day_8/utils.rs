#[derive(Debug)]
pub struct Tree {
    pub x: usize,
    pub y: usize,
    pub height: u8,
    pub is_visible_from_outside_x: bool,
    pub is_visible_from_outside_y: bool,
}

pub fn parse(input: &str) -> Vec<Vec<Tree>> {
    let mut tree_yard = Vec::new();
    let max_y = input.lines().count() - 1;
    let max_x = input.lines().next().unwrap().len() - 1;
    for (y, l) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in l.chars().enumerate() {
            let is_on_border = y == 0 || x == 0 || y == max_y || x == max_x;
            row.push(Tree {
                x,
                y,
                height: c.to_digit(10).unwrap() as u8,
                is_visible_from_outside_x: is_on_border,
                is_visible_from_outside_y: is_on_border,
            });
        }
        tree_yard.push(row);
    }
    return tree_yard;
}
