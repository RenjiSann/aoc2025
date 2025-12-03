use aoc;

struct Ticker {
    position: u32,
    count: u32,
}

impl Ticker {
    fn new() -> Self {
        Ticker {
            position: 50,
            count: 0,
        }
    }

    fn tick_right_n(&mut self, n: u32) {
        let new = self.position + n;
        self.count += new / 100;
        self.position = new % 100;
    }

    fn tick_left_n(&mut self, n: u32) {
        self.count += n / 100;
        let nn = n % 100;

        let x = self.position as i64 - nn as i64;

        if x <= 0 && self.position > 0 {
            self.count += 1;
        }

        self.position = x.rem_euclid(100) as u32;
    }
}

fn main() {
    let mut t = Ticker::new();

    for line in aoc::lines() {
        if line.is_empty() {
            break;
        }
        let side = line.as_bytes()[0];
        let distance = line[1..].parse::<u32>().unwrap();

        match side {
            b'R' => {
                let old = t.count;
                t.tick_right_n(distance);
                println!("{line:4} {:2} (+{:2})", t.position, t.count - old);
            }
            b'L' => {
                let old = t.count;
                t.tick_left_n(distance);
                println!("{line:4} {:2} (+{:2})", t.position, t.count - old);
            }
            _ => panic!("{side} is invalid"),
        };
    }

    println!("Answer: {}", t.count);
}
