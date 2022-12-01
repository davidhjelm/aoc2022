use std::{fs, io};

fn sum_calories(input: String) -> i32 {
    let mut raindeers: Vec<i32> = vec![0];
    let mut current = 0;

    for line in input.lines() {
        let calories = line.parse::<i32>();
        if calories.is_ok() {
            raindeers[current] += calories.unwrap();
        } else {
            raindeers.push(0);
            current += 1;
        }
    }

    *raindeers.iter().max().unwrap()
}

fn main() -> io::Result<()> {
    let data = fs::read_to_string("./inputs/d01").expect("Should open");

    println!("{}", sum_calories(data));

    Ok(())
}

#[test]
fn tfunc() {
    let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#
        .to_string();
    assert_eq!(sum_calories(input), 24000);
}
