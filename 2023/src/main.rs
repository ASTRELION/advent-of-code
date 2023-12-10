use std::{error::Error, time::Instant, env};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

fn main() -> Result<(), Box<dyn Error>>
{
    println!(r#"          .---_
         / / /\|
        / / | \ *
       /  /  \ \
      / /  / \  \
    ./~~~~~~~~~~~\.
   ( .",^. -". '.~ )
    '~~~~~~~~~~~~~'"#);
    println!("-----------------------");
    println!("| Advent of Code 2023 |");
    println!("|    by  ASTRELION    |");
    println!("-----------------------");

    let parts = [
        day1::part1, day1::part2,
        day2::part1, day2::part2,
        day3::part1, day3::part2,
        day4::part1, day4::part2,
        day5::part1, day5::part2,
        day6::part1, day6::part2,
        day7::part1, day7::part2,
        day8::part1, day8::part2,
        day9::part1, day9::part2,
        day10::part1, day10::part2,
    ];

    let mut args: Vec<_> = env::args().collect();
    args.drain(..1);

    for arg in &args
    {
        let day: i32 = arg
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap_or(0);

        if day > 0 && day <= parts.len() as i32 / 2
        {
            let day = (day - 1) as usize;
            run_day(day, [parts[day * 2], parts[(day * 2) + 1]]);
        }
    }

    if args.len() == 0
    {
        for day in 0..parts.len() / 2
        {
            run_day(day, [parts[day * 2], parts[(day * 2) + 1]]);
        }
    }

    return Ok(());
}

fn run_day(day_id: usize, parts: [fn() -> Result<i64, Box<dyn Error>>; 2])
{
    let start = Instant::now();
    let p1 = parts[0]();
    let time = Instant::now() - start;
    println!(
        "{:<24} {:<16} {:<16}",
        format!(
            "Day {}, Part 1 Solution:",
            day_id + 1
        ),
        p1.unwrap(),
        format!(
            "({:.4}ms)",
            time.as_nanos() as f64 / 1000000.0
        )
    );

    let start = Instant::now();
    let p2 = parts[1]();
    let time = Instant::now() - start;
    println!(
        "{:<24} {:<16} {:<16}",
        format!(
            "Day {}, Part 2 Solution:",
            day_id + 1
        ),
        p2.unwrap(),
        format!(
            "({:.4}ms)",
            time.as_nanos() as f64 / 1000000.0
        )
    );
}
