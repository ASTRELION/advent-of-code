use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn split_numbers(line: String) -> Vec<i64>
{
    return line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
}

pub fn part1() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day5/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.lines().collect();

    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut m: i64 = -1;

    for l in 2..lines.len()
    {
        let line = lines[l];
        if line.trim().is_empty()
        {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        if chars[0].is_digit(10)
        {
            maps[m as usize].push(split_numbers(line.to_string()));
        }
        else
        {
            maps.push(Vec::new());
            m += 1;
        }
    }

    let seeds = split_numbers(lines[0].replace("seeds: ", ""));
    let mut min_location = std::i64::MAX;

    for seed in seeds
    {
        let mut source = seed;
        for map in &maps
        {
            for range in map
            {
                if source >= range[1] && source < range[1] + range[2]
                {
                    source = range[0] + source - range[1];
                    break;
                }
            }
        }

        min_location = std::cmp::min(min_location, source);
    }

    println!("Day 5, Part 1 Solution: {}", min_location);
    return Ok(min_location);
}

pub fn part2() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day5/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.lines().collect();

    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut m: i64 = -1;

    for l in 2..lines.len()
    {
        let line = lines[l];
        if line.trim().is_empty()
        {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        if chars[0].is_digit(10)
        {
            maps[m as usize].push(split_numbers(line.to_string()));
        }
        else
        {
            maps.push(Vec::new());
            m += 1;
        }
    }

    // src len
    let seed_ranges = split_numbers(lines[0].replace("seeds: ", ""));
    let mut min_location = std::i64::MAX;

    // this only took my computer 92 minutes to run 😀
    // TODO: do it the actual way I should be doing this (mapping intervals to intervals)
    for s in (0..seed_ranges.len()).step_by(2)
    {
        for seed in seed_ranges[s]..(seed_ranges[s] + seed_ranges[s + 1] - 1)
        {
            let mut source = seed;
            for map in &maps
            {
                // dest src len
                for range in map
                {
                    if source >= range[1] && source < range[1] + range[2]
                    {
                        source = range[0] + source - range[1];
                        break;
                    }
                }
            }

            min_location = std::cmp::min(min_location, source);
        }
    }

    println!("Day 5, Part 2 Solution: {}", min_location);
    return Ok(min_location);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parts()
    {
        assert_eq!(part1().ok(), Some(35));
        assert_eq!(part2().ok(), Some(46));
    }
}
