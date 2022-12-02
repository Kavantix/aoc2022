use std::{time::Instant, fs::read_to_string};


pub fn run() {
    println!("Day 2");
    let _example = "A Y
B X
C Z";
    let input = read_to_string("input-day2.txt").unwrap();
    let start = Instant::now();
    let parsed = parse_input(&input);
    let elapsed = start.elapsed();
    println!("Parsing took: {:.2?}", elapsed);

    let start = Instant::now();
    let total_score = part_1(&parsed);
    let elapsed = start.elapsed();
    println!("Part 1 total score: {} (took {:.2?})", total_score, elapsed);

    let start = Instant::now();
    let total_score = part_2(&parsed);
    let elapsed = start.elapsed();
    println!("Part 2 total score: {} (took {:.2?})", total_score, elapsed);

}

fn parse_input(input: &str) -> Vec<(u8, u8)> {
    let letters = input.as_bytes();
    let mut i = 0;
    let mut result = Vec::with_capacity(letters.len());
    while i < letters.len() - 3 {
        result.push((letters[i] - b'A', letters[i + 2] - b'X'));
        i += 4
    }
    result
}

fn part_1(input: &Vec<(u8, u8)>) -> u32 {
    let mut total_score: u32 = 0;
    for (opponent_pick, my_pick) in input {
        total_score += (my_pick + 1 + 3 * ((my_pick - opponent_pick + 4) % 3)) as u32;
    }
    total_score
}

fn part_2(input: &Vec<(u8, u8)>) -> u32 {
    let mut total_score: u32 = 0;
    for (opponent_pick, outcome) in input {
        let outcome_score = outcome * 3;
        let my_pick = (opponent_pick + outcome - 1 + 3) % 3;
        total_score += (my_pick + 1 + outcome_score) as u32;
    }
    total_score
}
