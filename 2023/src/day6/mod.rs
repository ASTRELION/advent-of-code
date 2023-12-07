use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn split_numbers(line: String) -> Vec<i32>
{
    return line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
}

pub fn part1() -> Result<i32, Box<dyn Error>>
{
    let mut file = File::open("src/day6/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<_> = contents.lines().collect();
    let times = split_numbers(lines[0].replace("Time:", ""));
    let distances = split_numbers(lines[1].replace("Distance:", ""));

    let mut total = 1;

    for r in 0..times.len()
    {
        let t = times[r] as f64;
        let d = distances[r] as f64;

        // this is just the quadratic formula for
        // 0 = -x^2 + tx - d
        // where x is the time the button is held
        let x1 = (-t - (t.powf(2.0) - (4.0 * d)).sqrt()) / -2.0;
        let x2 = (-t + (t.powf(2.0) - (4.0 * d)).sqrt()) / -2.0;

        let mut delta = x1.floor() - x2.floor();
        if delta == x1 - x2
        {
            delta -= 1.0;
        }

        total *= delta as i32;
    }

    println!("Day 6, Part 1 Solution: {}", total);
    return Ok(total);
}

pub fn part2() -> Result<i32, Box<dyn Error>>
{
    let mut file = File::open("src/day6/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<_> = contents.lines().collect();
    let t = lines[0]
        .replace("Time:", "")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    let d = lines[1]
        .replace("Distance:", "")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    // this is just the quadratic formula for
    // 0 = -x^2 + tx - d
    // where x is the time the button is held
    let x1 = (-t - (t.powf(2.0) - (4.0 * d)).sqrt()) / -2.0;
    let x2 = (-t + (t.powf(2.0) - (4.0 * d)).sqrt()) / -2.0;

    let mut delta = x1.floor() - x2.floor();
    if delta == x1 - x2
    {
        delta -= 1.0;
    }

    println!("Day 6, Part 2 Solution: {}", delta as i32);
    return Ok(delta as i32);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parts()
    {
        assert_eq!(part1().ok(), Some(288));
        assert_eq!(part2().ok(), Some(71503));
    }
}
