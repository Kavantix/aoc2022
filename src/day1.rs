use std::fs::read_to_string;


pub fn run() {
    let _example = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

";
    use std::time::Instant;
    let start = Instant::now();
    let input = read_to_string("input-day1.txt").unwrap();
    let mut sum: i64 = 0;
    let mut third_highest_sum: i64 = 0;
    let mut second_highest_sum: i64 = 0;
    let mut highest_sum: i64 = 0;
    for line in input.lines() {
        let parsed_line = line.parse::<i64>();
        match parsed_line {
            Ok(number) => sum += number,
            Err(_) => { 
                if sum >= highest_sum {
                    third_highest_sum = second_highest_sum;
                    second_highest_sum = highest_sum;
                    highest_sum = sum;
                } else if sum >= second_highest_sum {
                    third_highest_sum = second_highest_sum;
                    second_highest_sum = sum;
                } else if sum >= third_highest_sum {
                    third_highest_sum = sum;
                }
                sum = 0;
            },
        }
    }
    println!("Highest calories: {}", highest_sum);
    println!("Second highest calories: {}", second_highest_sum);
    println!("Third highest calories: {}", third_highest_sum);
    println!("Total of three highest calories: {}", highest_sum + second_highest_sum + third_highest_sum);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
