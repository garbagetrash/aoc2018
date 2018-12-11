extern crate clap;

use clap::{App, Arg};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    let matches = App::new("AOC2018")
        .arg(
            Arg::with_name("DAY")
                .required(true)
                .index(1)
                .help("Day to run"),
        )
        .get_matches();

    let day = matches.value_of("DAY").unwrap().parse().unwrap();

    match day {
        1 => {
            let input = day01::day01::load_input();
            println!("Part 1 Solution: {}", day01::day01::part1(&input));
            println!("Part 2 Solution: {}", day01::day01::part2(&input));
        }
        2 => {
            let input = day02::day02::load_input();
            println!("Part 1 Solution: {}", day02::day02::part1(&input));
            println!("Part 2 Solution: {}", day02::day02::part2(&input));
        }
        3 => {
            let input = day03::day03::load_input();
            println!("Part 1 Solution: {}", day03::day03::part1(&input));
            println!("Part 2 Solution: {}", day03::day03::part2(&input).unwrap());
        }
        4 => {
            let input = day04::day04::load_input("inputs/04.txt");
            println!("Part 1 Solution: {}", day04::day04::part1(&input));
            println!("Part 2 Solution: {}", day04::day04::part2(&input));
        }
        5 => {
            let input = day05::day05::load_input("inputs/05.txt");
            println!("Part 1 Solution: {}", day05::day05::part1(&input));
            println!("Part 2 Solution: {}", day05::day05::part2(&input));
        }
        6 => {
            let input = day06::day06::load_input("inputs/06.txt");
            println!("Part 1 Solution: {}", day06::day06::part1(&input));
            println!("Part 2 Solution: {}", day06::day06::part2(&input, 10000));
        }
        7 => {
            let input = day07::day07::load_input("inputs/07.txt");
            println!("Part 1 Solution: {}", day07::day07::part1(&input));
            println!("Part 2 Solution: {}", day07::day07::part2(&input, 5, 60));
        }
        8 => {
            let input = day08::day08::load_input("inputs/08.txt");
            println!("Part 1 Solution: {}", day08::day08::part1(&input));
            println!("Part 2 Solution: {}", day08::day08::part2(&input));
        }
        9 => {
            let input = day09::day09::load_input("inputs/09.txt");
            println!("Part 1 Solution: {}", day09::day09::part1(&input));
            println!("Part 2 Solution: {}", day09::day09::part2(&input));
        }
        10 => {
            let input = day10::day10::load_input("inputs/10.txt");
            day10::day10::part1(&input);
        }
        11 => {
            let input = 9810;
            println!("Part 1 Solution: {:?}", day11::day11::part1(input));
            println!("Part 2 Solution: {:?}", day11::day11::part2(input));
        }
        _ => println!("Day {} not yet implemented", day),
    }
}
