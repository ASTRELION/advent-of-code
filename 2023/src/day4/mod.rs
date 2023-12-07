use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn part1() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day4/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total = 0;

    for line in contents.lines()
    {
        let card_split: Vec<&str> = line.split(": ").collect();

        let numbers: Vec<&str> = card_split[1].split(" | ").collect();
        let winners: Vec<i32> = numbers[0]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let have: Vec<i32> = numbers[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let count = winners.iter().filter(|w| have.contains(w)).count();
        if count > 0
        {
            total += 2_i32.pow(count as u32 - 1);
        }
    }

    return Ok(total as i64);
}

fn sum_card(card_map: &HashMap<i32, i32>, index: i32) -> i32
{
    let count = card_map.get(&index);
    if count == Some(&0) || count == None
    {
        return 0;
    }

    return
        (1..=*count.unwrap())
            .map(|i| sum_card(card_map, index + i))
            .sum::<i32>()
        + count.unwrap();
}

pub fn part2() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day4/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total: i32;
    let mut card_map: HashMap<i32, i32> = HashMap::new();

    for line in contents.lines()
    {
        let card_split: Vec<&str> = line.split(": ").collect();
        let card_id = card_split[0]
            .split_whitespace()
            .collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();

        let numbers: Vec<&str> = card_split[1].split(" | ").collect();
        let winners: Vec<i32> = numbers[0]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let have: Vec<i32> = numbers[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let count = winners.iter().filter(|w| have.contains(w)).count() as i32;
        card_map.insert(card_id, count);
    }

    total = card_map.keys().map(|k| sum_card(&card_map, *k)).sum();
    total += card_map.len() as i32;

    return Ok(total as i64);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parts()
    {
        assert_eq!(part1().ok(), Some(13));
        assert_eq!(part2().ok(), Some(30));
    }
}
