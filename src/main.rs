use std::process::Command;

mod days;

type Solution = fn(&str) -> (i64, i64);

static REGISTRY: &[Solution] = &[days::day_1::solution];

fn main() {
    let day = std::env::args().nth(1).expect("should specify day to run");

    let day = day
        .parse::<u64>()
        .expect("should specify a number for the day");

    download_day(day);

    let input = std::fs::read_to_string(format!("inputs/{day}.txt"))
        .expect("should be able to read input file");
    let result = REGISTRY[day as usize - 1](&input);
    println!("part 1: {}, part 2: {}", result.0, result.1);
}

fn download_day(day: u64) {
    let day_file = format!("inputs/{day}.txt");
    if std::fs::exists(&day_file).expect("should be able to tell if input exists") {
        return;
    }

    let token = std::fs::read_to_string("session")
        .expect("expect session file to exist containing session token for advent of code");

    let url = format!("https://adventofcode.com/2025/day/{day}/input");
    let cookie = format!("Cookie: session={token}");

    Command::new("curl").args([&url, "-H", &cookie, "-o", &day_file]);
}
