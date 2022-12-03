use std::{time::Instant, fs::read_to_string};


pub fn run() {
    println!("Day 3");
    let _example = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    let input = read_to_string("input-day3.txt").unwrap();
    // let input = _example;
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

fn parse_input(input: &str) -> Vec<([bool; 52], [bool; 52])> {
    input.lines().map(|line| {
        let mut lhs = [false; 52];
        let mut rhs = [false; 52];
        for i in 0..line.len() /2 {
            let letter = line.as_bytes()[i];
            if letter > b'Z' {
                lhs[(letter - b'a') as usize] = true
            } else {
                lhs[(letter - b'A' + 26) as usize] = true
            }
        }
        for i in line.len()/2..line.len() {
            let letter = line.as_bytes()[i];
            if letter > b'Z' {
                rhs[(letter - b'a') as usize] = true
            } else {
                rhs[(letter - b'A' + 26) as usize] = true
            }
        }
        (lhs, rhs)
    }).collect()
}

fn part_1(input: &Vec<([bool; 52], [bool; 52])>) -> u32 {
    input.iter()
        .map(|(lhs, rhs)| {
            let mut result = 0;
            for i in 0..52 {
                if lhs[i] && rhs[i] {
                    result = (i + 1) as u32;
                    break;
                }
            }
            result
        })
        .sum()
}

fn part_2(input: &Vec<([bool; 52], [bool; 52])>) -> u32 {
    {0..input.len()}.step_by(3)
        .map(|l| {
            let line1 = input[l+0];
            let line2 = input[l+1];
            let line3 = input[l+2];
            let mut result = 0;
            for i in 0..52 {
                if (line1.0[i] || line1.1[i])
                && (line2.0[i] || line2.1[i])
                && (line3.0[i] || line3.1[i]) {
                    result = (i + 1) as u32;
                    break;
                }
            }
            result
        })
        .sum()
}
