use std::str::FromStr;

pub struct ClearingTask {
    start: u16,
    stop: u16,
}

impl FromStr for ClearingTask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<u16> = s.split("-").map(|n| n.parse().unwrap()).collect();
        Ok(ClearingTask {
            start: split[0],
            stop: split[1],
        })
    }
}

impl ClearingTask {
    pub fn fully_contains(self: &ClearingTask, other: &ClearingTask) -> bool {
        self.start <= other.start && self.stop >= other.stop
    }

    pub fn overlaps(self: &ClearingTask, other: &ClearingTask) -> bool {
        self.start >= other.start && self.start <= other.stop ||
        self.stop >= other.start && self.stop <= other.stop ||
        self.fully_contains(other)
    }
}

pub fn parse(input: &str) -> Vec<(ClearingTask, ClearingTask)> {
    input
        .lines()
        .map(|l| {
            let pairs: Vec<&str> = l.split(",").collect();
            (
                ClearingTask::from_str(pairs[0]).unwrap(),
                ClearingTask::from_str(pairs[1]).unwrap()
            )
        }).collect()
}
