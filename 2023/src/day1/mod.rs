use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn part1() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day1/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut total = 0;
    for line in contents.lines()
    {
        let digit1 = line.chars().find(|c| c.is_digit(10)).unwrap();
        let digit2 = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
        total += format!("{}{}", digit1, digit2).parse::<i32>()?;
    }

    return Ok(total as i64);
}

pub fn part2() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day1/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut total = 0;
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];
    for line in contents.lines()
    {
        // first indexes of each digit
        let pattern_i = digits
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| line.find(p).map(|mi| (p, mi, i)));
        // minimum index pattern
        let mins = pattern_i
            .min_by_key(|&(_, mi, _)| mi)
            .unwrap();

        // last indexes of each digit
        let pattern_i = digits
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| line.rfind(p).map(|mi| (p, mi, i)));
        // maximum index pattern
        let maxs = pattern_i
            .max_by_key(|&(_, mi, _)| mi)
            .unwrap();

        total += ((((mins.2 % 9) + 1) * 10) + ((maxs.2 % 9) + 1)) as i32;
    }

    return Ok(total as i64);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parts()
    {
        assert_eq!(part1().ok(), Some(253));
        assert_eq!(part2().ok(), Some(281));
    }
}
