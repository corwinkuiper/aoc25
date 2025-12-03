fn part_1(input: &str) -> i64 {
    let mut s = 0;
    let lines = input.lines().map(|x| {
        x.chars()
            .map(|x| x.to_digit(10).unwrap() as i64)
            .collect::<Vec<_>>()
    });

    for line in lines {
        let &first = line[0..line.len() - 1].iter().max().unwrap();

        let pos_of_first = line.iter().position(|&x| x == first).unwrap();
        let second = line[pos_of_first + 1..].iter().max().unwrap();

        s += first * 10 + second;
    }

    s
}

fn part_2(input: &str) -> i64 {
    let mut s = 0;
    let lines = input.lines().map(|x| {
        x.chars()
            .map(|x| x.to_digit(10).unwrap() as i64)
            .collect::<Vec<_>>()
    });

    for line in lines {
        let mut p = 0;
        for digit in (0..12).rev() {
            let &this = line[p..line.len() - digit].iter().max().unwrap();
            let pos = line[p..].iter().position(|&x| x == this).unwrap();

            p += pos + 1;
            s += this * 10i64.pow(digit as u32);
        }
    }

    s
}

pub fn solution(input: &str) -> (i64, i64) {
    (part_1(input), part_2(input))
}
