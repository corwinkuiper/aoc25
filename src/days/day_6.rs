fn part_1(input: &str) -> i64 {
    let mut lines = input
        .lines()
        .map(|x| x.split_whitespace())
        .collect::<Vec<_>>();

    let mut s = 0i64;

    loop {
        let Some(operator) = lines.last_mut().unwrap().next() else {
            break;
        };

        let l = lines.len() - 1;
        let numbers = lines[..l]
            .iter_mut()
            .map(|x| x.next().unwrap().parse::<i64>().unwrap());

        s += match operator {
            "*" => numbers.product::<i64>(),
            "+" => numbers.sum(),
            _ => panic!("unexpected operator {operator}"),
        }
    }

    s
}

fn part_2(input: &str) -> i64 {
    let mut lines = input.lines().collect::<Vec<_>>();

    let mut operators = lines.pop().unwrap().split_whitespace().rev().peekable();

    let mut lines = lines
        .into_iter()
        .map(|x| x.chars().rev().peekable())
        .collect::<Vec<_>>();

    let mut s = 0;

    loop {
        if lines.iter_mut().all(|x| x.peek().is_none()) {
            break;
        }

        let mut numbers = Vec::new();

        while !lines
            .iter_mut()
            .all(|x| x.peek() == Some(&' ') || x.peek().is_none())
        {
            let n = lines
                .iter_mut()
                .flat_map(|x| x.next().unwrap().to_digit(10))
                .reduce(|acc, n| acc * 10 + n)
                .expect("should get number");
            numbers.push(n as i64);
        }

        lines.iter_mut().for_each(|x| {
            x.next();
        });

        s += match operators.next().expect("should get operator") {
            "*" => numbers.into_iter().product::<i64>(),
            "+" => numbers.into_iter().sum(),
            operator => panic!("unexpected operator {operator}"),
        }
    }

    s
}

pub fn solution(input: &str) -> (i64, i64) {
    (part_1(input), part_2(input))
}
