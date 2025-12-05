use std::{collections::HashSet, ops::RangeInclusive};

fn part_1(input: &str) -> i64 {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|x| {
            let (start, end) = x.split_once('-').unwrap();
            (start.parse::<i64>().unwrap())..=(end.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    ids.lines()
        .map(|x| x.parse::<i64>().unwrap())
        .filter(|&x| ranges.iter().any(|range| range.contains(&x)))
        .count() as i64
}

fn range_overlaps(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> bool {
    *a.start() <= *b.end() && *b.start() <= *a.end()
}

fn overlapping_range(
    a: &RangeInclusive<i64>,
    b: &RangeInclusive<i64>,
) -> Option<RangeInclusive<i64>> {
    if !range_overlaps(a, b) {
        return None;
    }

    Some((*a.start()).min(*b.start())..=(*a.end()).max(*b.end()))
}

fn part_2(input: &str) -> i64 {
    let (ranges, _) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|x| {
            let (start, end) = x.split_once('-').unwrap();
            (start.parse::<i64>().unwrap())..=(end.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut merged_ranges = HashSet::new();

    for mut range in ranges {
        loop {
            if let Some(mergable) = merged_ranges.iter().find(|&a| range_overlaps(a, &range)) {
                range =
                    overlapping_range(mergable, &range).expect("already found to be overlapping");

                merged_ranges.remove(&mergable.clone());
            } else {
                merged_ranges.insert(range);
                break;
            }
        }
    }

    merged_ranges
        .iter()
        .map(|x| x.end() - x.start() + 1)
        .sum::<i64>()
}

pub fn solution(input: &str) -> (i64, i64) {
    (part_1(input), part_2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_overlap() {
        assert!(range_overlaps(&(0..=1), &(1..=2)));
    }
}
