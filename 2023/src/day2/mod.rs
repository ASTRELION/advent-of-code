use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn part1() -> Result<i32, Box<dyn Error>>
{
    let mut file = File::open("src/day2/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let possible_cubes = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    let mut possible_games = 0;

    for line in contents.lines()
    {
        let game_split: Vec<&str> = line.split(": ").collect();
        let game_id = game_split[0]
            .split(" ")
            .collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let mut valid_game = true;

        let sets: Vec<&str> = game_split[1].split("; ").collect();
        for set in sets
        {
            let cubes: Vec<&str> = set.split(", ").collect();
            for cube in cubes
            {
                let cube_split: Vec<&str> = cube.split(" ").collect();
                let count: i32 = cube_split[0].parse().unwrap();
                let color = cube_split[1];
                if let Some(&value) = possible_cubes.get(color)
                {
                    if value < count
                    {
                        valid_game = false;
                        break;
                    }
                }
            }
        }

        if valid_game
        {
            possible_games += game_id;
        }
    }

    println!("Day 2, Part 1 Solution: {}", possible_games);
    return Ok(possible_games);
}

pub fn part2() -> Result<i32, Box<dyn Error>>
{
    let mut file = File::open("src/day2/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut power_sum = 0;

    for line in contents.lines()
    {
        let game_split: Vec<&str> = line.split(": ").collect();
        let mut min_cubes = HashMap::from([
            ("red", 1),
            ("green", 1),
            ("blue", 1)
        ]);

        let sets: Vec<&str> = game_split[1].split("; ").collect();
        for set in sets
        {
            let cubes: Vec<&str> = set.split(", ").collect();
            for cube in cubes
            {
                let cube_split: Vec<&str> = cube.split(" ").collect();
                let count: i32 = cube_split[0].parse().unwrap();
                let color = cube_split[1];
                min_cubes.insert(color, std::cmp::max(
                    min_cubes.get(color).unwrap().to_owned(),
                    count
                ));
            }
        }

        power_sum += min_cubes.values().fold(1, |acc, elem| acc * elem);
    }

    println!("Day 2, Part 2 Solution: {}", power_sum);
    return Ok(power_sum);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parts()
    {
        assert_eq!(part1().ok(), Some(8));
        assert_eq!(part2().ok(), Some(2286));
    }
}
