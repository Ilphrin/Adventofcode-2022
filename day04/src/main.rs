use std::str::FromStr;
#[derive(Clone, Copy, Debug)]
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
    fn fully_overlap(self, other: &Self) -> bool {
        if self.start >= other.start && self.end <= other.end {
            return true;
        }
        false
    }

    fn partially_overlap(self, other: &Self) -> bool {
        if (other.start..other.end + 1).contains(&self.start)
            || (other.start..other.end + 1).contains(&self.end)
        {
            return true;
        }
        false
    }
}

#[derive(Clone, Copy, Debug)]
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
        if self.first.fully_overlap(&self.second) {
            return 1;
        }
        if self.second.fully_overlap(&self.first) {
            return 1;
        }
        0
    }

    fn partially_overlap(self) -> i32 {
        if self.first.partially_overlap(&self.second) {
            return 1;
        }
        if self.second.partially_overlap(&self.first) {
            return 1;
        }
        0
    }
}

fn main() -> color_eyre::eyre::Result<()> {
    let sum: i32 = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Pair>().unwrap())
        .map(|assignment| assignment.partially_overlap())
        .sum();

    println!("Number of elves' assignment overlapping: {sum}");

    Ok(())
}
