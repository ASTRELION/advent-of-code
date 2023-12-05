mod day1;
mod day2;
mod day3;

fn main()
{
    println!(r#"          .---_
         / / /\|
        / / | \ *
       /  /  \ \
      / /  / \  \
    ./~~~~~~~~~~~\.
   ( .",^. -". '.~ )
    '~~~~~~~~~~~~~'"#);
    println!("-----------------------");
    println!("| Advent of Code 2023 |");
    println!("|    by  ASTRELION    |");
    println!("-----------------------");

    let _ = day1::part1();
    let _ = day1::part2();

    let _ = day2::part1();
    let _ = day2::part2();

    let _ = day3::part1();
    let _ = day3::part2();
}
