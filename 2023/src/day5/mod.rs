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

    let lines: Vec<_> = contents.lines().collect();

    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut m: i64 = -1;

    for l in 2..lines.len()
    {
        let line = lines[l];
        if line.trim().is_empty()
        {
            continue;
        }

        let chars: Vec<_> = line.chars().collect();
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

    // Maps of ranges in the form ((src_start, src_end), (dest_start, dest_end))
    let mut maps: Vec<Vec<((i64, i64), (i64, i64))>> = Vec::new();
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
            let split = split_numbers(line.to_string());
            // ingest ranges as ((src_start, src_end), (dest_start, dest_end))
            maps[m as usize].push((
                (split[1], split[1] + split[2] - 1),
                (split[0], split[0] + split[2] - 1)
            ));
        }
        else
        {
            maps.push(Vec::new());
            m += 1;
        }
    }

    let split = split_numbers(lines[0].replace("seeds: ", ""));
    // (start, end)
    let mut seed_ranges: Vec<_> = split.chunks(2).map(|c| (c[0], c[0] + c[1] - 1)).collect();
    let mut min_location = std::i64::MAX;

    for map in maps
    {
        let mut new_range: Vec<(i64, i64)> = Vec::new();
        for range in &seed_ranges
        {
            new_range.extend(&map_range(&map, range));
        }
        seed_ranges = new_range;
    }

    for range in seed_ranges
    {
        min_location = std::cmp::min(min_location, range.0);
    }

    println!("Day 5, Part 2 Solution: {}", min_location);
    return Ok(min_location);
}

fn map_range(map: &Vec<((i64, i64), (i64, i64))>, range: &(i64, i64)) -> Vec<(i64, i64)>
{
    let mut destinations: Vec<(i64, i64)> = Vec::new();

    for (src, dest) in map
    {
        // [ { } ]
        if range.0 >= src.0 && range.1 <= src.1
        {
            destinations.push((dest.0 + range.0 - src.0, dest.1 + range.1 - src.1));
        }
        // [ { ] }
        else if range.0 >= src.0 && range.0 <= src.1
        {
            let inner_range = (dest.0 + range.0 - src.0, dest.1);
            destinations.push(inner_range);
            destinations.extend(map_range(map, &(src.1 + 1, range.1)));
        }
        // { [ } ]
        else if range.1 <= src.1 && range.1 >= src.0
        {
            let inner_range = (dest.0, dest.0 + range.1 - src.0);
            destinations.push(inner_range);
            destinations.extend(map_range(map, &(range.0, src.0 - 1)));
        }
        // { } [ ]
        else {}
    }

    if destinations.len() == 0
    {
        destinations.push(*range);
    }

    return destinations;
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
