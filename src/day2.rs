use std::{fs::read_to_string, vec};


pub fn run() {
    println!("Day 2");
    let _example = "A Y
B X
C Z";
    use std::time::Instant;
    let start = Instant::now();
    let input = read_to_string("input-day2.txt").unwrap();

    part_1(&input);
    part_2(&input);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn part_1(input: &str) {
    let mut total_score = 0;
    for round in input.lines().map(|line| {
        let mut indices = vec![];
        let letters: Vec<char> = line.replace(" ", "").chars().collect();
        indices.push(letters[0] as u32 - 'A' as u32);
        indices.push(letters[1] as u32 - 'X' as u32);
        indices
    }) {
            if (round[0] + 1) % 3 == round[1] {
                // win
                total_score += 6 + round[1] + 1
            } else if round[0] == round[1] {
                // draw
                total_score += 3 + round[1] + 1
            } else {
                // loss
                total_score += round[1] + 1
            }
    }
    println!("Part 1 total score: {}", total_score);
}

fn part_2(input: &str) {
    let mut total_score = 0;
    for mut round in input.lines().map(|line| {
        let mut indices = vec![];
        let letters: Vec<char> = line.replace(" ", "").chars().collect();
        indices.push(letters[0] as i32 - 'A' as i32);
        indices.push(letters[1] as i32 - 'X' as i32);
        indices
    }) {
            round[1] = round[0] + round[1] - 1;
            if round[1] < 0 {
                round[1] = 2;
            } else if round[1] > 2 {
                round[1] = 0
            }
            if (round[0] + 1) % 3 == round[1] {
                // win
                total_score += 6 + round[1] + 1
            } else if round[0] == round[1] {
                // draw
                total_score += 3 + round[1] + 1
            } else {
                // loss
                total_score += round[1] + 1
            }
    }
    println!("Part 2 total score: {}", total_score);
}
