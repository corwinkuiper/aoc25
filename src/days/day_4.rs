use crate::util::Grid;

fn part_1(input: &str) -> i64 {
    let grid = Grid::new(input);

    grid.iter()
        .filter(|(_, roll)| *roll == b'@')
        .filter(|&((x, y), _)| {
            let surrounding = (-1..=1)
                .flat_map(move |x| (-1..=1).map(move |y| (x, y)))
                .filter(|&(x, y)| x != 0 || y != 0);
            surrounding
                .filter(|&(xx, yy)| grid.get(xx + x, yy + y) == Some(b'@'))
                .count()
                < 4
        })
        .count() as i64
}

fn part_2(input: &str) -> i64 {
    let mut grid = Grid::new(input);

    let mut removed = 0;

    loop {
        let to_remove = grid
            .iter()
            .filter(|(_, roll)| *roll == b'@')
            .filter(|&((x, y), _)| {
                let surrounding = (-1..=1)
                    .flat_map(move |x| (-1..=1).map(move |y| (x, y)))
                    .filter(|&(x, y)| x != 0 || y != 0);
                surrounding
                    .filter(|&(xx, yy)| grid.get(xx + x, yy + y) == Some(b'@'))
                    .count()
                    < 4
            })
            .map(|(pos, _)| pos)
            .collect::<Vec<_>>();

        if to_remove.is_empty() {
            break removed;
        }

        removed += to_remove.len() as i64;

        for (x, y) in to_remove {
            grid.set(x, y, b'.');
        }
    }
}

pub fn solution(input: &str) -> (i64, i64) {
    (part_1(input), part_2(input))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_grid() {
        let grid = Grid::new(
            "#-
01
",
        );

        assert_eq!(grid.get(-1, 0), None);
        assert_eq!(grid.get(0, 0), Some(b'#'));
        assert_eq!(grid.get(0, 1), Some(b'0'));
        assert_eq!(grid.get(2, 0), None);
    }
}
