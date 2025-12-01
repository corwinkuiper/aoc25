fn part_1(input: &str) -> i64 {
    let dial_movements = input.lines().map(|line| {
        let n = line[1..].parse::<i64>().expect("value should be number");

        if line.starts_with('L') { -n } else { n }
    });

    let mut dial_position = 50;
    let mut zero_counts = 0;

    for movement in dial_movements {
        dial_position += movement;
        dial_position = dial_position.rem_euclid(100);
        if dial_position == 0 {
            zero_counts += 1;
        }
    }

    zero_counts
}

fn part_2(input: &str) -> i64 {
    let dial_movements = input.lines().map(|line| {
        let n = line[1..].parse::<i64>().expect("value should be number");

        if line.starts_with('L') { -n } else { n }
    });

    let mut dial_position = 50;
    let mut zero_counts = 0;

    for movement in dial_movements {
        let s = movement.signum();

        for _ in 0..movement.abs() {
            dial_position += s;

            dial_position = dial_position.rem_euclid(100);

            if dial_position == 0 {
                zero_counts += 1;
            }
        }
    }

    zero_counts
}

pub fn solution(input: &str) -> (i64, i64) {
    (part_1(input), part_2(input))
}
