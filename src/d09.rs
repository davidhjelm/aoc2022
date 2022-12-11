use std::{fs, io};

#[derive(PartialEq, Copy, Clone)]
struct Pos(i32, i32);

struct Rope {
    pieces: Vec<Pos>,
    tail_positions: Vec<Pos>,
}

impl Rope {
    fn new(n: i32) -> Self {
        let mut pieces: Vec<Pos> = vec![];

        for _i in 0..n {
            pieces.push(Pos(0, 0));
        }

        Self {
            pieces,
            tail_positions: vec![],
        }
    }

    fn parse(&mut self, input: &str) {
        let mut parsed = input.split(" ");
        let dir = parsed.next().unwrap();
        let amount: i32 = parsed.next().unwrap().parse().unwrap();
        let add_pos;
        match dir {
            "R" => add_pos = Pos(0, 1),
            "L" => add_pos = Pos(0, -1),
            "U" => add_pos = Pos(-1, 0),
            "D" => add_pos = Pos(1, 0),
            _ => panic!("should not get here"),
        }

        for _i in 0..amount {
            self.move_head(add_pos); // move head

            for i in 1..self.pieces.len() {
                self.update_rope(i, i - 1);

                if i + 1 == self.pieces.len() {
                    let p = &self.pieces[i];
                    if !self.tail_positions.contains(&p) {
                        self.tail_positions.push(*p);
                    }
                }
            }
        }
    }

    fn is_touching(&self, start: usize, end: usize) -> bool {
        if self.pieces[start].0 - self.pieces[end].0 < 2
            && self.pieces[start].0 - self.pieces[end].0 > -2
            && self.pieces[start].1 - self.pieces[end].1 < 2
            && self.pieces[start].1 - self.pieces[end].1 > -2
        {
            return true;
        } else {
            return false;
        }
    }

    fn move_head(&mut self, add_pos: Pos) {
        self.pieces[0].0 += add_pos.0;
        self.pieces[0].1 += add_pos.1;
    }

    fn get_dir(&self, this: usize, prev: usize) -> Pos {
        let mut new_dir = Pos(0, 0);
        if self.pieces[prev].0 < self.pieces[this].0 {
            new_dir.0 -= 1;
        }
        if self.pieces[prev].0 > self.pieces[this].0 {
            new_dir.0 += 1;
        }
        if self.pieces[prev].1 < self.pieces[this].1 {
            new_dir.1 -= 1;
        }
        if self.pieces[prev].1 > self.pieces[this].1 {
            new_dir.1 += 1;
        }
        new_dir
    }

    fn update_rope(&mut self, this: usize, prev: usize) {
        if !self.is_touching(this, prev) {
            let new_dir = self.get_dir(this, prev);
            self.pieces[this].0 += new_dir.0;
            self.pieces[this].1 += new_dir.1;
        }
    }
}

fn main() -> io::Result<()> {
    let data = fs::read_to_string("./inputs/d09").expect("Should open");

    // pt 1:
    let mut rope1 = Rope::new(2);

    for line in data.lines() {
        rope1.parse(line);
    }

    // pt 2:
    let mut rope2 = Rope::new(10);

    for line in data.lines() {
        rope2.parse(line);
    }

    println!(
        "pt1: {}, pt2: {}",
        rope1.tail_positions.len(),
        rope2.tail_positions.len()
    );

    Ok(())
}

#[test]
fn test_part2() {
    let mut rope = Rope::new(2);
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

    let mut rope = Rope::new(10);

    let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    for line in input.lines() {
        rope.parse(line);
    }

    assert_eq!(rope.tail_positions.len(), 36);
}
