use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;



fn label_value(label: &char, wild_j: bool) -> i32
{
    let mut labels = [
        'A', 'K', 'Q', 'J', 'T',
        '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    if wild_j
    {
        labels = [
            'A', 'K', 'Q', 'T',
            '9', '8', '7', '6', '5', '4', '3', '2', 'J'
        ];
    }

    return (labels.len() - labels.iter().position(|&l| l == *label).unwrap()) as i32;
}

fn hand_value(hand: &str, wild_j: bool) -> i32
{
    let mut map: HashMap<char, i32> = HashMap::new();

    for char in hand.chars()
    {
        map
            .entry(char)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    if wild_j
    {
        let j_count = *map.get(&'J').unwrap_or(&0);
        let max_char = map
            .iter()
            .filter(|&c| c.0 != &'J')
            .max_by(|a, b| a.1.cmp(b.1));

        if let Some(not_j_max) = max_char
        {
            if j_count > 0
            {
                map.entry(*not_j_max.0).and_modify(|c| *c += j_count);
                map.remove(&'J');
            }
        }
    }

    let max = map.values().max().unwrap();
    let key_count = map.keys().count();

    match (key_count, max)
    {
        (1, _) => return 7, // five of a kind
        (2, 4) => return 6, // four of a kind
        (2, 3) => return 5, // full house
        (3, 3) => return 4, // three of a kind
        (3, 2) => return 3, // two pair
        (4, 2) => return 2, // one pair
        (5, 1) => return 1, // high card
        _ => return 0,      // everything else
    }
}

fn compare_hands(hand1: &str, hand2: &str, wild_j: bool) -> i32
{
    let rank1 = hand_value(hand1, wild_j);
    let rank2 = hand_value(hand2, wild_j);
    if rank1 != rank2
    {
        return rank1 - rank2;
    }

    for l in 0..hand1.len()
    {
        let value1 = label_value(&hand1.chars().nth(l).unwrap(), wild_j);
        let value2 = label_value(&hand2.chars().nth(l).unwrap(), wild_j);
        if value1 != value2
        {
            return value1 - value2;
        }
    }

    return 1;
}

fn binary_insert(hand_bids: &Vec<(&str, i32)>, hand: &str, wild_j: bool) -> usize
{
    let mut a = 0;
    let mut b = hand_bids.len();

    while a < b
    {
        let mid = a + ((b - a) / 2);
        if compare_hands(hand, hand_bids[mid].0, wild_j) > 0
        {
            a = mid + 1;
        }
        else
        {
            b = mid;
        }
    }

    return a;
}

pub fn part1() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day7/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut hand_bids: Vec<(&str, i32)> = Vec::new();

    for line in contents.lines()
    {
        let hand_bid: Vec<_> = line.split(" ").collect();
        let hand = hand_bid[0];
        let bid: i32 = hand_bid[1].parse().unwrap();

        // binary insert is ~26x faster (~45ms) for the full input
        // but in this case a linear insert would be quick enough (~1.2s) too
        let i = binary_insert(&hand_bids, hand, false);
        hand_bids.insert(i, (hand, bid));
    }

    let score = hand_bids
        .iter()
        .enumerate()
        .fold(0, |a, e| a + ((e.0 + 1) as i32 * e.1.1));

    return Ok(score as i64);
}

pub fn part2() -> Result<i64, Box<dyn Error>>
{
    let mut file = File::open("src/day7/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut hand_bids: Vec<(&str, i32)> = Vec::new();

    for line in contents.lines()
    {
        let hand_bid: Vec<_> = line.split(" ").collect();
        let hand = hand_bid[0];
        let bid: i32 = hand_bid[1].parse().unwrap();

        // binary insert is ~26x faster (~45ms) for the full input
        // but in this case a linear insert would be quick enough (~1.2s) too
        let i = binary_insert(&hand_bids, hand, true);
        hand_bids.insert(i, (hand, bid));
    }

    let score = hand_bids
        .iter()
        .enumerate()
        .fold(0, |a, e| a + ((e.0 + 1) as i32 * e.1.1));

    return Ok(score as i64);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parts()
    {
        assert_eq!(part1().ok(), Some(6440));
        assert_eq!(part2().ok(), Some(5905));
    }
}
