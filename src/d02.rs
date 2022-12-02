use std::{fs, io};

#[derive(Debug, PartialEq, Clone, Copy)]
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

fn hand_matching_outcome(opponent: Hand, outcome: Outcome) -> Hand {
    match opponent {
        Hand::Rock => match outcome {
            Outcome::Win => Hand::Paper,
            Outcome::Draw => Hand::Rock,
            Outcome::Loss => Hand::Scissors,
        },
        Hand::Paper => match outcome {
            Outcome::Draw => Hand::Paper,
            Outcome::Loss => Hand::Rock,
            Outcome::Win => Hand::Scissors,
        },
        Hand::Scissors => match outcome {
            Outcome::Draw => Hand::Scissors,
            Outcome::Win => Hand::Rock,
            Outcome::Loss => Hand::Paper,
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

fn char_to_hand(input: char) -> Hand {
    match input {
        'A' | 'X' => Hand::Rock,
        'B' | 'Y' => Hand::Paper,
        'C' | 'Z' => Hand::Scissors,
        _ => panic!(),
    }
}

fn char_to_outcome(input: char) -> Outcome {
    match input {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!(),
    }
}

fn string_to_hands(input: &str) -> (Hand, Hand) {
    let inputs: Vec<&str> = input.split(' ').collect();
    let opp_hand: char = inputs[0].chars().last().unwrap();
    let own_hand: char = inputs[1].chars().last().unwrap();

    (char_to_hand(opp_hand), char_to_hand(own_hand))
}

fn string_to_hand_and_outcome(input: &str) -> (Hand, Outcome) {
    let inputs: Vec<&str> = input.split(' ').collect();
    let opp_hand: char = inputs[0].chars().last().unwrap();
    let wanted_outcome: char = inputs[1].chars().last().unwrap();

    (char_to_hand(opp_hand), char_to_outcome(wanted_outcome))
}

fn main() -> io::Result<()> {
    let data = fs::read_to_string("./inputs/d02").expect("Should open");

    let mut sum_pt1 = 0;
    let mut sum_pt2 = 0;

    for line in data.lines() {
        let hands = string_to_hands(line);
        let hand_and_outcome = string_to_hand_and_outcome(line);
        let outcome = hand_matching_outcome(hand_and_outcome.0, hand_and_outcome.1);
        sum_pt1 += count_score(hands.0, hands.1);
        sum_pt2 += count_score(hands.0, outcome)
    }

    println!("Total score Part 1: {}, Part 2: {}", sum_pt1, sum_pt2);

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

    assert_eq!(
        string_to_hand_and_outcome(input[0]),
        (Hand::Rock, Outcome::Draw)
    );

    assert_eq!(
        string_to_hand_and_outcome(input[1]),
        (Hand::Paper, Outcome::Loss)
    );

    assert_eq!(
        string_to_hand_and_outcome(input[2]),
        (Hand::Scissors, Outcome::Win)
    );

    assert_eq!(hand_matching_outcome(Hand::Rock, Outcome::Draw), Hand::Rock);
    assert_eq!(
        hand_matching_outcome(Hand::Paper, Outcome::Loss),
        Hand::Rock
    );
    assert_eq!(
        hand_matching_outcome(Hand::Scissors, Outcome::Win),
        Hand::Rock
    );

    assert_eq!(count_score(Hand::Rock, Hand::Rock), 4);
}
