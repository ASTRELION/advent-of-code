use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn part1() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day8/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<_> = contents.lines().collect();

    let instructions: Vec<_> = lines[0].split("").filter(|c| c.len() > 0).collect();
    let mut map: HashMap<&str, (String, String)> = HashMap::new();

    for i in 2..lines.len()
    {
        let line = lines[i];
        let key_split: Vec<_> = line.split(" = ").collect();
        let key = key_split[0];
        let to: Vec<_> = key_split[1]
            .split(", ")
            .map(|t| t.replace("(", "").replace(")", ""))
            .collect();
        map.insert(key, (to[0].clone(), to[1].clone()));
    }

    let mut current_node = "AAA";
    let mut i = 0;

    while current_node != "ZZZ"
    {
        for instruction in &instructions
        {
            i += 1;
            match instruction
            {
                &"L" =>
                {
                    current_node = &map[current_node].0;
                }
                &"R" =>
                {
                    current_node = &map[current_node].1;
                }
                _ => {}
            }

            if current_node == "ZZZ"
            {
                break;
            }
        }
    }

    return Ok(i);
}

fn find_z(map: &HashMap<&str, (String, String)>, instructions: &Vec<&str>, node: &str) -> i64
{
    let mut current_node = node;
    let mut d = 0;
    while !current_node.ends_with("Z")
    {
        for instruction in instructions
        {
            d += 1;
            match instruction
            {
                &"L" =>
                {
                    current_node = &map[current_node].0;
                }
                &"R" =>
                {
                    current_node = &map[current_node].1;
                }
                _ => {}
            }

            if current_node.ends_with("Z")
            {
                break;
            }
        }
    }

    return d;
}

fn lcm(a: i64, b: i64) -> i64
{
    let mut x = a;
    let mut y = b;
    if x < y
    {
        x = b;
        y = a;
    }

    let mut remainder = x % y;

    while remainder != 0
    {
        x = y;
        y = remainder;
        remainder = x % y;
    }

    return a * b / y;
}

pub fn part2() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day8/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<_> = contents.lines().collect();

    let instructions: Vec<_> = lines[0].split("").filter(|c| c.len() > 0).collect();
    let mut map: HashMap<&str, (String, String)> = HashMap::new();
    let mut starters = HashSet::new();

    for i in 2..lines.len()
    {
        let line = lines[i];
        let key_split: Vec<_> = line.split(" = ").collect();
        let key = key_split[0];
        if key.ends_with("A")
        {
            starters.insert(key);
        }

        let to: Vec<_> = key_split[1]
            .split(", ")
            .map(|t| t.replace("(", "").replace(")", ""))
            .collect();
        map.insert(key, (to[0].clone(), to[1].clone()));
    }

    let d: Vec<_> = starters
        .iter()
        .map(|s| find_z(&map, &instructions, s))
        .collect();

    let mut common = d[0];
    for i in 1..d.len()
    {
        common = lcm(common, d[i]);
    }

    return Ok(common as i64);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parts()
    {
        assert_eq!(part1().ok(), Some(2));
        assert_eq!(part2().ok(), Some(2));
    }
}
