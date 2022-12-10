use std::str::FromStr;
#[derive(Clone, Copy)]
struct Assignment {
    start: i32,
    end: i32,
}

impl FromStr for Assignment {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s.split('-').collect::<Vec<&str>>();

        Ok(Self {
            start: vals[0].parse::<i32>().unwrap(),
            end: vals[1].parse::<i32>().unwrap(),
        })
    }
}

impl Assignment {
    fn overlap_with(self, other: &Self) -> bool {
        if self.start >= other.start && self.end <= other.end {
            return true;
        }
        false
    }
}

struct Pair {
    first: Assignment,
    second: Assignment,
}

impl FromStr for Pair {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s.split(',').collect::<Vec<&str>>();

        Ok(Self {
            first: vals[0].parse::<Assignment>().unwrap(),
            second: vals[1].parse::<Assignment>().unwrap(),
        })
    }
}

impl Pair {
    fn fully_overlap(self) -> i32 {
        if self.first.overlap_with(&self.second) {
            return 1;
        }
        if self.second.overlap_with(&self.first) {
            return 1;
        }
        0
    }
}

fn main() -> color_eyre::eyre::Result<()> {
    let sum: i32 = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Pair>().unwrap())
        .map(|assignment| assignment.fully_overlap())
        .sum();

    println!("Number of elves' assignment overlapping: {sum}");

    Ok(())
}
