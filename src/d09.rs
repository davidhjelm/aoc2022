#![allow(dead_code)]

use std::{fs, io};

#[derive(PartialEq, Clone, Copy, Debug)]
struct Pos(i32, i32);

struct Rope {
    head: Pos,
    tail: Pos,
    tail_positions: Vec<Pos>,
}

impl Rope {
    fn is_touching(&self) -> bool {
        if self.head.0 - self.tail.0 < 2
            && self.head.0 - self.tail.0 > -2
            && self.head.1 - self.tail.1 < 2
            && self.head.1 - self.tail.1 > -2
        {
            return true;
        } else {
            return false;
        }
    }

    fn move_rope(&mut self, add_pos: Pos) {
        self.head.0 += add_pos.0;
        self.head.1 += add_pos.1;
    }

    fn update_rope(&mut self, add_pos: Pos) {
        if !self.is_touching() {
            self.tail.0 += add_pos.0;
            self.tail.1 += add_pos.1;

            if add_pos.0 != 0 {
                self.tail.1 = self.head.1
            } else if add_pos.1 != 0 {
                self.tail.0 = self.head.0
            }
        }
        if !self.tail_positions.contains(&self.tail) {
            self.tail_positions.push(self.tail);
        }
    }

    fn parse(&mut self, input: &str) {
        let mut parsed = input.split(" ");
        let dir = parsed.next().unwrap();
        let amount: i32 = parsed.next().unwrap().parse().unwrap();
        let pos;
        match dir {
            "R" => pos = Pos(0, 1),
            "L" => pos = Pos(0, -1),
            "U" => pos = Pos(-1, 0),
            "D" => pos = Pos(1, 0),
            _ => panic!("should not get here"),
        }

        for _i in 0..amount {
            self.move_rope(pos);
            self.update_rope(pos);
        }
    }
}

fn main() -> io::Result<()> {
    let data = fs::read_to_string("./inputs/d09").expect("Should open");

    let mut rope = Rope {
        head: Pos(0, 0),
        tail: Pos(0, 0),
        tail_positions: vec![],
    };

    for line in data.lines() {
        rope.parse(line);
    }

    dbg!(rope.tail_positions.len());

    Ok(())
}

#[test]
fn test_move() {
    let mut rope = Rope {
        head: Pos(0, 0),
        tail: Pos(0, 0),
        tail_positions: vec![],
    };

    rope.move_rope(Pos(0, 1));

    assert_eq!(rope.head.0, 0);
    assert_eq!(rope.head.1, 1);

    assert_eq!(rope.is_touching(), true);
    rope.move_rope(Pos(0, 1));
    assert_eq!(rope.is_touching(), false);
    rope.update_rope(Pos(0, 1));
    assert_eq!(rope.is_touching(), true);

    let mut rope = Rope {
        head: Pos(-1, 1),
        tail: Pos(0, 0),
        tail_positions: vec![],
    };
    assert_eq!(rope.is_touching(), true);
    rope.move_rope(Pos(-1, 0));
    assert_eq!(rope.is_touching(), false);
    rope.update_rope(Pos(-1, 0));

    assert_eq!(rope.tail.0 == -1 && rope.tail.1 == 1, true);

    let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    for line in input.lines() {
        rope.parse(line);
    }

    assert_eq!(rope.tail_positions.len(), 13);
}

fn test_is_touching() {
    let rope = Rope {
        head: Pos(0, 0),
        tail: Pos(0, 0),
        tail_positions: vec![],
    };

    assert_eq!(rope.is_touching(), true);

    let rope = Rope {
        head: Pos(1, 0),
        tail: Pos(0, 0),
        tail_positions: vec![],
    };

    assert_eq!(rope.is_touching(), true);

    let rope = Rope {
        head: Pos(1, 1),
        tail: Pos(0, 0),
        tail_positions: vec![],
    };

    assert_eq!(rope.is_touching(), true);

    let rope = Rope {
        head: Pos(0, 1),
        tail: Pos(-1, 0),
        tail_positions: vec![],
    };

    assert_eq!(rope.is_touching(), true);

    let rope = Rope {
        head: Pos(1, 1),
        tail: Pos(-1, 0),
        tail_positions: vec![],
    };

    assert_eq!(rope.is_touching(), false);

    let rope = Rope {
        head: Pos(1, 2),
        tail: Pos(0, 0),
        tail_positions: vec![],
    };

    assert_eq!(rope.is_touching(), false);
}
