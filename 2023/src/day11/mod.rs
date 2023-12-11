use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn find_galactic_distances(contents: String, expansion: i64) -> i64
{
    let mut galaxy: Vec<Vec<char>> = Vec::new();
    let mut galaxy_coords: HashSet<(i64, i64)> = HashSet::new();
    let mut occupied_row: HashSet<usize> = HashSet::new();
    let mut occupied_col: HashSet<usize> = HashSet::new();

    // find occupied rows/columns for expansion
    for l_line in contents.lines().enumerate()
    {
        galaxy.push(Vec::new());
        for c_char in l_line.1.chars().enumerate()
        {
            galaxy[l_line.0].push(c_char.1);
            if c_char.1 == '#'
            {
                occupied_col.insert(c_char.0);
                occupied_row.insert(l_line.0);
            }
        }
    }

    // find coordinates of galaxies relative to expansion
    let mut dx = 0;
    let mut dy = 0;

    for galactic_row in galaxy.iter().enumerate()
    {
        dx = 0;
        if !occupied_row.contains(&galactic_row.0)
        {
            dy += expansion;
        }

        for galactic_col in galactic_row.1.iter().enumerate()
        {
            if !occupied_col.contains(&galactic_col.0)
            {
                dx += expansion;
            }

            if galactic_col.1 == &'#'
            {
                galaxy_coords.insert((dx + galactic_col.0 as i64, dy + galactic_row.0 as i64));
            }
        }
    }

    // find distances
    let mut sum = 0;
    let galaxy_coords: Vec<_> = galaxy_coords.iter().collect();

    for i in 0..galaxy_coords.len()
    {
        for j in i + 1..galaxy_coords.len()
        {
            sum += (galaxy_coords[i].0 - galaxy_coords[j].0).abs() + (galaxy_coords[i].1 - galaxy_coords[j].1).abs();
        }
    }

    return sum;
}

pub fn part1() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day11/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(find_galactic_distances(contents, 1));
}

pub fn part2() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day11/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(find_galactic_distances(contents, 999999));
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parts()
    {
        assert_eq!(part1().ok(), Some(374));
        assert_eq!(part2().ok(), Some(82000210));
    }
}
