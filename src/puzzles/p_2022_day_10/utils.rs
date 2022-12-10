use std::str::FromStr;

#[derive(Debug)]
pub enum Instructions {
    Noop { cycles: u8 },
    Addx { cycles: u8, add: i32 },
}

impl FromStr for Instructions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Instructions::Noop { cycles: 1 }),
            s => {
                let nbr = s.split(' ').last().unwrap().parse().unwrap();
                Ok(Instructions::Addx {
                    cycles: 2,
                    add: nbr,
                })
            }
        }
    }
}

pub fn parse(input: &str) -> Vec<Instructions> {
    input
        .lines()
        .map(|l| Instructions::from_str(l).unwrap())
        .collect()
}
