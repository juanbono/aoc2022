use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Rucksack {
    first: HashSet<char>,
    second: HashSet<char>,
}

impl Rucksack {
    pub fn new(input_line: &str) -> Option<Rucksack> {
        let char_vec = input_line.chars().collect_vec();
        let mut halves = char_vec.chunks_exact(input_line.len() / 2 as usize);
        let first_half = halves.next()?.into_iter();
        let second_half = halves.next()?.into_iter();

        let mut first = HashSet::new();
        for c in first_half {
            first.insert(c.to_owned());
        }
        let mut second = HashSet::new();
        for c in second_half {
            second.insert(c.to_owned());
        }

        Some(Rucksack { first, second })
    }

    pub fn find_wrong_items(&self) -> Vec<char> {
        self.first
            .intersection(&self.second)
            .map(|c| c.to_owned())
            .collect_vec()
    }

    pub fn priority(c: char) -> u32 {
        if c.is_lowercase() {
            (c as u32) - 96
        } else {
            (c as u32) - 38
        }
    }

    pub fn get_priorities(&self) -> u32 {
        self.find_wrong_items()
            .into_iter()
            .map(|c| Rucksack::priority(c))
            .sum()
    }
}

struct Group {
    first: HashSet<char>,
    second: HashSet<char>,
    third: HashSet<char>,
}

impl Group {
    pub fn priority(&self) -> u32 {
        let mut total = 0;
        let first_intersect: HashSet<char> = self
            .first
            .intersection(&self.second)
            .map(|c| c.to_owned())
            .collect();
        let second_intersect = first_intersect.intersection(&self.third);

        for item in second_intersect {
            total += Rucksack::priority(item.to_owned());
        }

        total
    }
}
struct Badges(Vec<Group>);

impl Badges {
    pub fn new(reader: impl BufRead) -> Badges {
        let mut groups = vec![];

        let lines = reader.lines().map(|l| l.unwrap()).collect_vec();
        let chunks = lines.chunks_exact(3).collect_vec();

        for chunk in chunks {
            let chunk = chunk.to_owned();
            let first = HashSet::from_iter(chunk[0].chars());
            let second = HashSet::from_iter(chunk[1].chars());
            let third = HashSet::from_iter(chunk[2].chars());

            let group = Group {
                first,
                second,
                third,
            };

            groups.push(group);
        }
        Badges(groups)
    }

    pub fn total(self) -> u32 {
        let mut total = 0;

        for group in self.0 {
            total += group.priority();
        }

        total
    }
}

fn main() -> std::io::Result<()> {
    let input_file = File::open("inputs/input3")?;
    let reader = BufReader::new(input_file);
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some(rucksack) = Rucksack::new(&line) {
            total += rucksack.get_priorities();
        }
    }

    println!("Part 1: {total}");

    let input_file = File::open("inputs/input3")?;
    let reader = BufReader::new(input_file);
    let badges = Badges::new(reader);

    let part2 = badges.total();

    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Rucksack;

    #[test]
    fn can_find_wrong_items() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::new(input).unwrap();
        assert_eq!(vec!['p'], rucksack.find_wrong_items());

        let input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let rucksack = Rucksack::new(input).unwrap();
        assert_eq!(vec!['L'], rucksack.find_wrong_items());

        let input = "PmmdzqPrVvPwwTWBwg";
        let rucksack = Rucksack::new(input).unwrap();
        assert_eq!(vec!['P'], rucksack.find_wrong_items());

        let input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let rucksack = Rucksack::new(input).unwrap();
        assert_eq!(vec!['v'], rucksack.find_wrong_items());

        let input = "ttgJtRGJQctTZtZT";
        let rucksack = Rucksack::new(input).unwrap();
        assert_eq!(vec!['t'], rucksack.find_wrong_items());

        let input = "CrZsJsPPZsGzwwsLwLmpwMDw";
        let rucksack = Rucksack::new(input).unwrap();
        assert_eq!(vec!['s'], rucksack.find_wrong_items());
    }
}
