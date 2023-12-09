use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn diffs(history: &Vec<i64>) -> Vec<i64>
{
    return history
        .windows(2)
        .map(|p| p[1] - p[0])
        .collect();
}

fn find_next(history: &Vec<i64>) -> i64
{
    if history.iter().all(|&h| h == 0)
    {
        return 0;
    }

    return history.last().unwrap() + find_next(&diffs(history));
}

fn find_previous(history: &Vec<i64>) -> i64
{
    if history.iter().all(|&h| h == 0)
    {
        return 0;
    }

    return history.first().unwrap() - find_previous(&diffs(history));
}

pub fn part1() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day9/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;

    for line in contents.lines()
    {
        let history: Vec<i64> = line
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect();
        sum += find_next(&history);
    }

    return Ok(sum);
}

pub fn part2() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day9/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;

    for line in contents.lines()
    {
        let history: Vec<i64> = line
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect();
        sum += find_previous(&history);
    }

    return Ok(sum);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parts()
    {
        assert_eq!(part1().ok(), Some(114));
        assert_eq!(part2().ok(), Some(2));
    }
}
