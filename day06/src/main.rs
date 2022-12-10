const MARKER_START: usize = 14;

fn find_marker(input: &str) -> usize {
    input
        .as_bytes()
        .windows(MARKER_START)
        .position(|window| {
            let mut arr = [false; 256];
            for &item in window {
                if arr[item as usize] {
                    return false;
                }
                arr[item as usize] = true;
            }
            true
        })
        .map(|pos| pos + MARKER_START)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::find_marker;

    #[test]
    fn test_fin_marker() {
        assert_eq!(7, find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    }
}

fn main() {
    let line = include_str!("input.txt");

    println!("Marker: {:?}", find_marker(line));
}
