use std::fs;

fn first_last_digit(s: &str) -> Option<(u64, u64)> {
    let chars = s.chars().collect::<Vec<char>>();

    for c in &chars {
        if let Some(v) = c.to_digit(10) {
            let first = v as u64;
            for c in chars.iter().rev() {
                if let Some(v) = c.to_digit(10) {
                    let last = v as u64;
                    return Some((first, last));
                }
            }
        }
    }

    None
}

fn part_one_sum(data: &str) -> u64 {
    let mut sum = 0;
    for line in data.lines() {
        if let Some((first, last)) = first_last_digit(line) {
            let value = first * 10 + last;
            //println!("{line}: ({first}, {last})={value}");
            sum += value;
        } else {
            panic!("error: no digits in {}", line);
        }
    }

    sum
}

fn part_two_sum(_data: &str) -> u64 {
    let sum = 0;

    // todo

    sum
}

fn main() {
    let example_1: String = fs::read_to_string("data/example_1.txt").unwrap();
    let example_2: String = fs::read_to_string("data/example_2.txt").unwrap();
    let input: String = fs::read_to_string("data/input.txt").unwrap();

    println!("part one:");
    println!("    example 1: {}", part_one_sum(&example_1));
    //println!("    example 2: {}", part_one_sum(&example_2));
    println!("    input:     {}", part_one_sum(&input));
    println!("");

    println!("part two:");
    println!("    example 1: {}", part_two_sum(&example_1));
    println!("    example 2: {}", part_two_sum(&example_2));
    println!("    input:     {}", part_two_sum(&input));
    println!("");

}
