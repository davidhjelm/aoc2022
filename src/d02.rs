use std::{fs, io};

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

fn is_win(opponent: Hand, own: Hand) -> Outcome {
    match opponent {
        Hand::Rock => match own {
            Hand::Rock => Outcome::Draw,
            Hand::Paper => Outcome::Win,
            Hand::Scissors => Outcome::Loss,
        },
        Hand::Paper => match own {
            Hand::Paper => Outcome::Draw,
            Hand::Scissors => Outcome::Win,
            Hand::Rock => Outcome::Loss,
        },
        Hand::Scissors => match own {
            Hand::Scissors => Outcome::Draw,
            Hand::Rock => Outcome::Win,
            Hand::Paper => Outcome::Loss,
        },
    }
}

fn count_score(opponent: Hand, own: Hand) -> i32 {
    let mut sum = 0;

    match own {
        Hand::Rock => sum += 1,
        Hand::Paper => sum += 2,
        Hand::Scissors => sum += 3,
    }

    match is_win(opponent, own) {
        Outcome::Win => sum += 6,
        Outcome::Draw => sum += 3,
        Outcome::Loss => sum += 0,
    }

    sum
}

fn to_hand(input: char) -> Hand {
    match input {
        'A' | 'X' => Hand::Rock,
        'B' | 'Y' => Hand::Paper,
        'C' | 'Z' => Hand::Scissors,
        _ => panic!(),
    }
}

fn string_to_hands(input: &str) -> (Hand, Hand) {
    let hands: Vec<&str> = input.split(' ').collect();
    let opp_hand: char = hands[0].chars().last().unwrap();
    let own_hand: char = hands[1].chars().last().unwrap();

    (to_hand(opp_hand), to_hand(own_hand))
}

fn main() -> io::Result<()> {
    let data = fs::read_to_string("./inputs/d02").expect("Should open");

    let mut sum = 0;
    for line in data.lines() {
        let hands = string_to_hands(line);
        sum += count_score(hands.0, hands.1);
    }

    println!("Total score: {}", sum);

    Ok(())
}

#[test]
fn tfunc() {
    let input = ["A Y", "B X", "C Z"];

    assert_eq!(string_to_hands(input[0]), (Hand::Rock, Hand::Paper));
    assert_eq!(string_to_hands(input[1]), (Hand::Paper, Hand::Rock));
    assert_eq!(string_to_hands(input[2]), (Hand::Scissors, Hand::Scissors));

    assert_eq!(is_win(Hand::Rock, Hand::Paper), Outcome::Win);

    assert_eq!(count_score(Hand::Rock, Hand::Paper), 8);
}
