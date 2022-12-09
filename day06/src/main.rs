use itertools::Itertools;

fn main() {
    let line = include_str!("input.txt")
        .lines()
        .next()
        .expect("To be able to read lines");
    println!("{line}");
    for (index, char) in line.chars().enumerate() {
        let slice = line.chars().skip(index).peek_nth(2);
    }
}
