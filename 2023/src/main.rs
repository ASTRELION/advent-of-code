use std::{error::Error, time::Instant};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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
    ];

    for p in (0..parts.len()).step_by(2)
    {
        let start = Instant::now();
        let p1 = parts[p]();
        let time = Instant::now() - start;
        println!(
            "{:<24} {:<16} {:<16}",
            format!(
                "Day {}, Part 1 Solution:",
                (p / 2) + 1
            ),
            p1.unwrap(),
            format!(
                "({:.4}ms)",
                time.as_nanos() as f64 / 1000000.0
            )
        );

        let start = Instant::now();
        let p2 = parts[p + 1]();
        let time = Instant::now() - start;
        println!(
            "{:<24} {:<16} {:<16}",
            format!(
                "Day {}, Part 2 Solution:",
                (p / 2) + 1
            ),
            p2.unwrap(),
            format!(
                "({:.4}ms)",
                time.as_nanos() as f64 / 1000000.0
            )
        );
    }

    return Ok(());
}
