use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn part1() -> Result<i32, Box<dyn Error>>
{
    let mut file = File::open("src/day3/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // pad string with "empty" lines of .
    let mut lines: Vec<&str> = contents.lines().collect();
    let empty = ".".repeat(lines[0].len());
    lines.push(&empty);
    lines.insert(0, &empty);

    let mut total = 0;

    for l in 1..lines.len() - 1
    {
        let pline = format!(".{}.", lines[l - 1]);
        let line = format!(".{}.", lines[l]);
        let nline = format!(".{}.", lines[l + 1]);

        let mut digits = String::new();
        let mut valid_num = false;

        let pchars: Vec<char> = pline.chars().collect();
        let chars: Vec<char> = line.chars().collect();
        let nchars: Vec<char> = nline.chars().collect();

        for i in 1..line.len() - 1
        {
            let mut set_valid = false;
            match pchars[i]
            {
                '.' => {},
                '0'..='9' => {},
                _ => { valid_num = true; set_valid = true; },
            }
            match nchars[i]
            {
                '.' => {},
                '0'..='9' => {},
                _ => { valid_num = true; set_valid = true; },
            }
            match chars[i]
            {
                '.' =>
                {
                    if valid_num && digits.len() > 0
                    {
                        total += digits.parse::<i32>().unwrap();
                    }
                    valid_num = if !set_valid { false } else { valid_num };
                    digits = String::new();
                },
                '0'..='9' =>
                {
                    digits += &chars[i].to_string();
                },
                _ =>
                {
                    valid_num = true;
                    if valid_num && digits.len() > 0
                    {
                        total += digits.parse::<i32>().unwrap();
                    }
                    digits = String::new();
                },
            }

        }

        if valid_num && digits.len() > 0
        {
            total += digits.parse::<i32>().unwrap();
        }
    }

    println!("Day 3, Part 1 Solution: {}", total);
    return Ok(total);
}

pub fn bisect(chars: &Vec<char>, start: usize) -> String
{
    let mut digit_string = String::new();
    let mut left_broken = false;
    let mut right_broken = false;

    for d in 0..chars.len()
    {
        if let (Some(&l), true) = (chars.get(start.saturating_sub(d)), !left_broken)
        {
            match l
            {
                '0'..='9' => digit_string = l.to_string() + &digit_string,
                _ if d != 0 => left_broken = true,
                _ => digit_string = l.to_string() + &digit_string,
            }
        }

        if let (Some(&r), true) = (chars.get(start + 1 + d), !right_broken)
        {
            match r
            {
                '0'..='9' => digit_string = digit_string + &r.to_string(),
                _ => right_broken = true,
            }
        }
    }

    return digit_string;
}

pub fn part2() -> Result<i32, Box<dyn Error>>
{
    let mut file = File::open("src/day3/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut lines: Vec<&str> = contents.lines().collect();
    let empty = ".".repeat(lines[0].len());
    lines.push(&empty);
    lines.insert(0, &empty);
    let mut total = 0;

    for l in 1..lines.len() - 1
    {
        let chars: Vec<char> = format!(".{}.", lines[l]).chars().collect();
        let pchars: Vec<char> = format!(".{}.", lines[l - 1]).chars().collect();
        let nchars: Vec<char> = format!(".{}.", lines[l + 1]).chars().collect();

        for c in 1..chars.len() - 1
        {
            let char = chars[c];
            if char != '*'
            {
                continue;
            }

            let mut parts: Vec<String> = Vec::new();
            parts.extend(
                bisect(&pchars, c)
                    .split(|c: char| !c.is_digit(10))
                    .map(String::from)
            );
            parts.extend(
                bisect(&chars, c)
                    .split(|c: char| !c.is_digit(10))
                    .map(String::from)
            );
            parts.extend(
                bisect(&nchars, c)
                    .split(|c: char| !c.is_digit(10))
                    .map(String::from)
            );
            parts = parts
                .iter()
                .filter(|&s| s.len() > 0)
                .cloned()
                .collect();

            if parts.len() == 2
            {
                total += parts
                    .iter()
                    .fold(1, |a, c| a * c.parse::<i32>()
                    .unwrap());
            }
        }
    }

    println!("Day 3, Part 2 Solution: {}", total);
    return Ok(total);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parts()
    {
        assert_eq!(part1().ok(), Some(4361));
        assert_eq!(part2().ok(), Some(467835));
    }
}
