use std::{fs::read_to_string, vec};
use std::time::Instant;


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

    part_1(&parsed);
    part_2(&parsed);

}

fn parse_input(input: &str) -> Vec<[i32; 2]> {
    let letters = input.as_bytes();
    let mut rounds = letters.len() / 4;
    if letters.len() % 4 != 0 {
        rounds += 1;
    }
    let mut result = Vec::with_capacity(rounds);
    for i in 0..rounds {
        result.push([letters[i*4] as i32, letters[i*4 + 2] as i32]);
    }
    result
}

fn part_1(input: &Vec<[i32; 2]>) {
    let start = Instant::now();
    let mut total_score = 0;
    let mut round = vec![0, 0];
    for letters in input {
        round[0] = letters[0]  - b'A' as i32;
        round[1] = letters[1]  - b'X' as i32;
        total_score += round[1] + 1 + 3 * ((round[1] - round[0] + 4) % 3);
    }
    let elapsed = start.elapsed();
    println!("Part 1 total score: {} (took {:.2?})", total_score, elapsed);
}

fn part_2(input: &Vec<[i32; 2]>) {
    let start = Instant::now();
    let mut total_score = 0;
    let mut round = vec![0, 0];
    for letters in input {
        round[0] = letters[0]  - b'A' as i32;
        round[1] = letters[1]  - b'X' as i32;
        round[1] = (round[0] + round[1] - 1 + 3) % 3;
        total_score += round[1] + 1 + 3 * ((round[1] - round[0] + 4) % 3);
    }
    let elapsed = start.elapsed();
    println!("Part 2 total score: {} (took {:.2?})", total_score, elapsed);
}
