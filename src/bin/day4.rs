use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct AssignmentPair(RangeInclusive<u32>, RangeInclusive<u32>);

impl AssignmentPair {
    pub fn is_fully_contained(&self) -> bool {
        let set1 = self.0.clone().collect::<HashSet<u32>>();
        let set2 = self.1.clone().collect::<HashSet<u32>>();

        set1.is_subset(&set2) || set2.is_subset(&set1)
    }

    pub fn has_overlap(&self) -> bool {
        let set1 = self.0.clone().collect::<HashSet<u32>>();
        let set2 = self.1.clone().collect::<HashSet<u32>>();

        !set1.is_disjoint(&set2)
    }
}

#[derive(Debug)]
struct ParsingError;

impl FromStr for AssignmentPair {
    type Err = ParsingError;

    /// Parses a &str like "2-4,6-8"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted_text = s.split(",");
        if let (Some(fst), Some(snd)) = (splitted_text.next(), splitted_text.next()) {
            let mut splitted_fst = fst.split("-");
            let mut splitted_snd = snd.split("-");

            if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (
                splitted_fst.next(),
                splitted_fst.next(),
                splitted_snd.next(),
                splitted_snd.next(),
            ) {
                let x1_u32 = x1.parse().map_err(|_| ParsingError)?;
                let y1_u32 = y1.parse().map_err(|_| ParsingError)?;
                let x2_u32 = x2.parse().map_err(|_| ParsingError)?;
                let y2_u32 = y2.parse().map_err(|_| ParsingError)?;

                return Ok(AssignmentPair(
                    RangeInclusive::new(x1_u32, y1_u32),
                    RangeInclusive::new(x2_u32, y2_u32),
                ));
            }
        }

        Err(ParsingError)
    }
}

fn main() -> io::Result<()> {
    let input_file = File::open("inputs/input4")?;
    let reader = BufReader::new(input_file);
    let mut amount_of_contained_assignments = 0;
    let mut overlaps = 0;

    for line in reader.lines() {
        if let Ok(assignment_pair) = line?.parse::<AssignmentPair>() {
            if assignment_pair.is_fully_contained() {
                amount_of_contained_assignments += 1;   
            }

            if assignment_pair.has_overlap() {
                overlaps += 1;
            }
        }
    }

    println!("Part 1: {amount_of_contained_assignments}");
    println!("Part 2: {overlaps}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use crate::AssignmentPair;

    #[test]
    fn can_parse_assignment_pairs() {
        let input = "2-4,6-8";

        let expected = AssignmentPair(RangeInclusive::new(2, 4), RangeInclusive::new(6, 8));

        assert_eq!(expected, input.parse().unwrap());
    }

    #[test]
    fn can_check_containment() {
        // the assignments are not contained.
        let input = "2-4,6-8";
        let assignment_pair: AssignmentPair = input.parse().unwrap();
        assert!(!assignment_pair.is_fully_contained());

        // the second assignment is fully contained in the first one.
        let input = "2-8,3-7";
        let assignment_pair: AssignmentPair = input.parse().unwrap();
        assert!(assignment_pair.is_fully_contained());
    }
}
