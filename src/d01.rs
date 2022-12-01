use std::{fs, io};

fn sum_calories(input: String, top_elves: i32) -> i32 {
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

    // reverse order
    raindeers.sort_by(|a, b| b.cmp(a));

    let sum = raindeers
        .get(0..top_elves as usize)
        .unwrap()
        .iter()
        .sum::<i32>();

    sum
}

fn main() -> io::Result<()> {
    let data = fs::read_to_string("./inputs/d01").expect("Should open");

    // pt 1
    println!("{}", sum_calories(data.clone(), 1));
    // pt 2
    println!("{}", sum_calories(data, 3));

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
    // pt 1
    assert_eq!(sum_calories(input.clone(), 1), 24000);

    // pt 2
    assert_eq!(sum_calories(input, 3), 45000);
}
