use std::fs;

// check if each character is a digit character
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

// checks if the given string slice starts with a digit
fn digit_at_start(s: &str) -> Option<u64> {
    if s.starts_with("zero") || s.starts_with("0") {
        Some(0)
    } else if s.starts_with("one") || s.starts_with("1") {
        Some(1)
    } else if s.starts_with("two") || s.starts_with("2") {
        Some(2)
    } else if s.starts_with("three") || s.starts_with("3") {
        Some(3)
    } else if s.starts_with("four") || s.starts_with("4") {
        Some(4)
    } else if s.starts_with("five") || s.starts_with("5") {
        Some(5)
    } else if s.starts_with("six") || s.starts_with("6") {
        Some(6)
    } else if s.starts_with("seven") || s.starts_with("7") {
        Some(7)
    } else if s.starts_with("eight") || s.starts_with("8") {
        Some(8)
    } else if s.starts_with("nine") || s.starts_with("9") {
        Some(9)
    } else {
        None
    }
}

// find the first and last numbers in a string including when they're spelled out
// note: aho-corasick is probably a better fit here
fn first_last_digit_improved(s: &str) -> Option<(u64, u64)> {
    let mut first = None;
    let mut last = None;

    for i in 0..s.len() {
        let remainder = &s[i..];
        if let Some(digit) = digit_at_start(remainder) {
            if first == None {
                first = Some(digit);
            }
            last = Some(digit)
        }
    }

    if first == None {
        None
    } else {
        Some((first.unwrap(), last.unwrap()))
    }
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

fn part_two_sum(data: &str) -> u64 {
    let mut sum = 0;
    for line in data.lines() {
        if let Some((first, last)) = first_last_digit_improved(line) {
            let value = first * 10 + last;
            //println!("{line}: ({first}, {last})={value}");
            sum += value;
        } else {
            panic!("error: no digits in {}", line);
        }
    }

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
