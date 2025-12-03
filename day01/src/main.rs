use aoc;

fn main() {
    let mut count = 0;
    let mut dial = 50;

    for line in aoc::lines() {
        if line.is_empty() {
            break;
        }
        let side = line.as_bytes()[0];
        let distance = line[1..].parse::<u32>().unwrap();

        match side {
            b'R' => {
                dial += distance % 100;
            }
            b'L' => {
                dial += 100 - distance % 100;
            }
            _ => panic!("{side} is invalid"),
        }
        dial %= 100;
        println!("State: {dial}");
        if dial == 0 {
            count += 1;
        }
    }

    println!("Answer: {count}");
}
