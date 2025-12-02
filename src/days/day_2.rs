fn to_digits(n: i64) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect()
}

fn part_1(input: &str) -> i64 {
    let ranges = input
        .split(',')
        .map(|x| x.trim().split_once('-').expect("should be a range"));

    let mut invalid_count = 0;

    for (start, end) in ranges {
        let s = start.parse::<i64>().unwrap();
        let e = end.parse::<i64>().unwrap();

        for i in s..=e {
            let n = to_digits(i);
            let first = &n[0..(n.len() / 2)];
            let second = &n[n.len() / 2..];
            if n.len().is_multiple_of(2) && first == second {
                invalid_count += i;
            }
        }
    }
    invalid_count
}

fn part_2(input: &str) -> i64 {
    let ranges = input
        .split(',')
        .map(|x| x.trim().split_once('-').expect("should be a range"));

    let mut invalid_count = 0;

    for (start, end) in ranges {
        let s = start.parse::<i64>().unwrap();
        let e = end.parse::<i64>().unwrap();

        for i in s..=e {
            let n = to_digits(i);
            for ws in 0..=n.len() / 2 {
                if !n.len().is_multiple_of(ws) {
                    continue;
                }
                let first = n.chunks(ws).next().unwrap();

                if n.chunks(ws).all(|x| x == first) {
                    invalid_count += i;
                    break;
                }
            }
        }
    }
    invalid_count
}

pub fn solution(input: &str) -> (i64, i64) {
    (part_1(input), part_2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_to_digits() {
        assert_eq!(to_digits(102), vec![1, 0, 2]);
    }
}
