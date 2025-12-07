use std::collections::{HashMap, HashSet};

use crate::util::Grid;

fn find_start(grid: &Grid) -> i64 {
    for x in 0..grid.width as i64 {
        if grid.get(x, 0) == Some(b'S') {
            return x;
        }
    }

    panic!("No start found");
}

fn part_1(input: &str) -> i64 {
    let grid = Grid::new(input);
    let start = find_start(&grid);

    let mut beams = HashSet::new();
    beams.insert(start);

    let mut split_count = 0;
    let mut y = 1;

    while y < grid.height as i64 {
        let mut next_beam = HashSet::new();

        for beam_point in beams {
            if grid.get(beam_point, y) == Some(b'^') {
                next_beam.insert(beam_point - 1);
                next_beam.insert(beam_point + 1);
                split_count += 1;
            } else {
                next_beam.insert(beam_point);
            }
        }

        beams = next_beam;

        y += 1;
    }

    split_count
}

fn part_2(input: &str) -> i64 {
    let grid = Grid::new(input);
    let start = find_start(&grid);

    fn count_many_world(x: i64, y: i64, cache: &mut HashMap<(i64, i64), i64>, grid: &Grid) -> i64 {
        if let Some(&result) = cache.get(&(x, y)) {
            return result;
        }

        if y > grid.height as i64 {
            return 1;
        }

        let result = if grid.get(x, y) == Some(b'^') {
            count_many_world(x - 1, y + 1, cache, grid)
                + count_many_world(x + 1, y + 1, cache, grid)
        } else {
            count_many_world(x, y + 1, cache, grid)
        };

        cache.insert((x, y), result);
        result
    }

    count_many_world(start, 0, &mut HashMap::new(), &grid)
}

pub fn solution(input: &str) -> (i64, i64) {
    (part_1(input), part_2(input))
}
