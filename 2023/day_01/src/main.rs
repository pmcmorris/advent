use std::error::Error;
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

fn main() -> Result<(), Box<dyn Error>> {
    let data: String = fs::read_to_string("data/input.txt")?;
    let mut sum = 0;
    for line in data.lines() {
        if let Some((first, last)) = first_last_digit(line) {
            let value = first * 10 + last;
            println!("{line}: ({first}, {last})={value}");
            sum += value;
        } else {
            panic!("error: no digits");
        }
    }

    println!("sum: {sum}");
    Ok(())
}
