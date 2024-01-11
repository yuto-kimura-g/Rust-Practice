use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        query: [(i32, String); q]
    }
    let mut dragon = VecDeque::new();
    for i in 1..=n {
        dragon.push_back(Position { x: i as i32, y: 0 });
    }
    for (t, cp) in query.iter() {
        match t {
            1 => {
                // 1, c: move the head by 1 in dir c.
                let c = cp;
                let head = dragon.get(0).unwrap().move_to(c);
                dragon.push_front(head);
                dragon.pop_back();
            }
            2 => {
                // 2, p: print the position of part p.
                let p = cp.parse::<usize>().unwrap() - 1; // 0-indexed
                let ans = dragon.get(p).unwrap(); // O(??)
                println!("{} {}", ans.x, ans.y);
            }
            _ => unreachable!("invalid query"),
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn move_to(&self, dir: &str) -> Position {
        match dir {
            "R" => Position {
                x: self.x + 1,
                y: self.y,
            },
            "L" => Position {
                x: self.x - 1,
                y: self.y,
            },
            "U" => Position {
                x: self.x,
                y: self.y + 1,
            },
            "D" => Position {
                x: self.x,
                y: self.y - 1,
            },
            _ => unreachable!("invalid direction"),
        }
    }
}
