use std::{fs, io};

fn find_double(input: String) -> Result<char, String> {
    let first = &input[0..input.len() / 2];
    let second = &input[input.len() / 2..];

    for c in first.chars() {
        if second.contains(c) {
            return Ok(c);
        }
    }

    Err("No double found".to_string())
}

fn get_priority(input: char) -> u8 {
    if input.is_uppercase() {
        input as u8 - 38
    } else {
        input as u8 - 96
    }
}

fn main() -> io::Result<()> {
    let data = fs::read_to_string("./inputs/d03").expect("Should open");

    let mut sum: i32 = 0;

    for line in data.lines() {
        let prio_char = find_double(line.to_string());
        sum += get_priority(prio_char.unwrap()) as i32;
    }

    println!("Total score: {}", sum);

    Ok(())
}

#[test]
fn tfunc() {
    assert_eq!(find_double("vJrwpWtwJgWrhcsFMMfFFhFp".to_string()), Ok('p'));

    assert_eq!(get_priority('p'), 16);
    assert_eq!(get_priority('L'), 38);
}
