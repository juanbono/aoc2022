use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Window;

impl Window {
    pub fn find_marker(input: &str) -> usize {
        for (fst, snd, thrd, fourth) in input.chars().enumerate().tuple_windows() {
            let set = HashSet::from([fst.1, snd.1, thrd.1, fourth.1]);
            if set.len() == 4 {
                return fourth.0 + 1;
            }
        }
        0
    }

    pub fn find_message_marker(input: &str) -> usize {
        for window in input.chars().enumerate().collect_vec().windows(14) {
            let set: HashSet<char> = window.iter().map(|(_i, c)| c.to_owned()).collect();
            if set.len() == 14 {
                // return the index of the end of the current window.
                return window.last().unwrap().0 + 1;
            }
        }
        0
    }
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("inputs/input6")?;
    let part1 = Window::find_marker(&input);
    println!("Part 1: {part1}");

    let part2 = Window::find_message_marker(&input);
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Window;

    #[test]
    fn can_find_first_mark() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, Window::find_marker(input));

        let input2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, Window::find_marker(input2));

        let input3 = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, Window::find_marker(input3));

        let input4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(10, Window::find_marker(input4));

        let input5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(11, Window::find_marker(input5));
    }

    #[test]
    fn can_find_message_marker() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, Window::find_message_marker(input));

        let input2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(23, Window::find_message_marker(input2));

        let input3 = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(23, Window::find_message_marker(input3));

        let input4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(29, Window::find_message_marker(input4));

        let input5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(26, Window::find_message_marker(input5));
    }
}
