use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Point
{
    x: i64,
    y: i64,
}

fn find_pipes(pipes: &Vec<Vec<char>>, start: &Point, current: &Point) -> HashSet<Point>
{
    let mut tiles: HashSet<Point> = HashSet::new();
    tiles.insert(*start);

    if current.y < 0 || current.y >= pipes.len() as i64 || current.x < 0 || current.y >= pipes[0].len() as i64
    {
        return tiles;
    }

    let mut c = *current;
    let mut p = *start;
    while c != *start
    {
        tiles.insert(c);
        let mut neighbours: Vec<Point> = Vec::new();
        match pipes[c.y as usize][c.x as usize]
        {
            '|' =>
            {
                if c.y > 0
                {
                    neighbours.push(Point { x: c.x, y: c.y - 1 });
                }
                if c.y < pipes.len() as i64 - 1
                {
                    neighbours.push(Point { x: c.x, y: c.y + 1 });
                }
            }
            '-' =>
            {
                if c.x > 0
                {
                    neighbours.push(Point { x: c.x - 1, y: c.y });
                }
                if c.x < pipes[0].len() as i64 - 1
                {
                    neighbours.push(Point { x: c.x + 1, y: c.y });
                }
            }
            'L' =>
            {
                if c.y > 0
                {
                    neighbours.push(Point { x: c.x, y: c.y - 1 });
                }
                if c.x < pipes[0].len() as i64 - 1
                {
                    neighbours.push(Point { x: c.x + 1, y: c.y });
                }
            }
            'J' =>
            {
                if c.y > 0
                {
                    neighbours.push(Point { x: c.x, y: c.y - 1 });
                }
                if c.x > 0
                {
                    neighbours.push(Point { x: c.x - 1, y: c.y });
                }
            }
            '7' =>
            {
                if c.x > 0
                {
                    neighbours.push(Point { x: c.x - 1, y: c.y });
                }
                if c.y < pipes.len() as i64 - 1
                {
                    neighbours.push(Point { x: c.x, y: c.y + 1 });
                }
            }
            'F' =>
            {
                if c.x < pipes[0].len() as i64 - 1
                {
                    neighbours.push(Point { x: c.x + 1, y: c.y });
                }
                if c.y < pipes.len() as i64 - 1
                {
                    neighbours.push(Point { x: c.x, y: c.y + 1 });
                }
            }
            _ => {}
        }

        if neighbours.len() == 0
        {
            break;
        }

        let temp = c;
        if neighbours[0] == p
        {
            c = neighbours[1];
        }
        else
        {
            c = neighbours[0];
        }
        p = temp;
    }
    return tiles;
}

pub fn part1() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day10/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut pipes: Vec<Vec<char>> = Vec::new();
    let mut start_r = 0;
    let mut start_c = 0;

    for line in contents.lines()
    {
        let numbers: Vec<char> = line
            .split("")
            .filter(|s| s.len() > 0)
            .map(|s| s.chars().nth(0).unwrap())
            .collect();
        let s = numbers
            .iter()
            .enumerate()
            .find(|n| *n.1 == 'S');
        if let Some(s) = s
        {
            start_c = s.0 as i64;
            start_r = pipes.len() as i64;
        }
        pipes.push(numbers);
    }

    let binding = [
        find_pipes(&pipes, &Point { x: start_c, y: start_r }, &Point { x: start_c + 1, y: start_r }).len(),
        find_pipes(&pipes, &Point { x: start_c, y: start_r }, &Point { x: start_c - 1, y: start_r }).len(),
        find_pipes(&pipes, &Point { x: start_c, y: start_r }, &Point { x: start_c, y: start_r + 1 }).len(),
        find_pipes(&pipes, &Point { x: start_c, y: start_r }, &Point { x: start_c, y: start_r - 1 }).len(),
    ];
    let d = binding.iter().max().unwrap();

    return Ok(*d as i64 / 2);
}

pub fn part2() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day10/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut pipes: Vec<Vec<char>> = Vec::new();
    let mut start_r = 0;
    let mut start_c = 0;

    for line in contents.lines()
    {
        let numbers: Vec<char> = line
            .split("")
            .filter(|s| s.len() > 0)
            .map(|s| s.chars().nth(0).unwrap())
            .collect();
        let s = numbers
            .iter()
            .enumerate()
            .find(|n| *n.1 == 'S');
        if let Some(s) = s
        {
            start_c = s.0 as i64;
            start_r = pipes.len() as i64;
        }
        pipes.push(numbers);
    }

    let mut tiles: HashSet<Point> = HashSet::new();
    tiles = tiles.union(
        &find_pipes(&pipes, &Point { x: start_c, y: start_r }, &Point { x: start_c + 1, y: start_r })
    ).map(|p| *p).collect();
    tiles = tiles.union(
        &find_pipes(&pipes, &Point { x: start_c, y: start_r }, &Point { x: start_c - 1, y: start_r })
    ).map(|p| *p).collect();
    tiles = tiles.union(
        &find_pipes(&pipes, &Point { x: start_c, y: start_r }, &Point { x: start_c, y: start_r + 1 })
    ).map(|p| *p).collect();
    tiles = tiles.union(
        &find_pipes(&pipes, &Point { x: start_c, y: start_r }, &Point { x: start_c, y: start_r - 1 })
    ).map(|p| *p).collect();

    let mut inside_count = 0;

    for y in 0..pipes.len()
    {
        let mut is_inside = false;
        for x in 0..pipes[y].len()
        {
            let c = pipes[y][x];
            let is_pipe = tiles.contains(&Point { x: x as i64, y: y as i64 });
            if (c == '|' || c == 'F' || c == '7' || c == 'S' ) && is_pipe
            {
                is_inside = !is_inside;
            }
            if is_inside && !is_pipe
            {
                inside_count += 1;
            }
        }
    }

    return Ok(inside_count);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parts()
    {
        assert_eq!(part1().ok(), Some(4));
        assert_eq!(part2().ok(), Some(1));
    }
}
