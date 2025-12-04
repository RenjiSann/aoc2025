use aoc;

fn process_line(s: &[u8]) -> u32 {
    assert!(s.len() > 1);
    let mut idx_tenth = 0;
    for (i, &e) in s[..s.len() - 1].iter().enumerate() {
        if e == b'9' {
            idx_tenth = i;
            break;
        }

        if e > s[idx_tenth] {
            idx_tenth = i;
        }
    }

    let mut idx_unit = idx_tenth + 1;
    for (i, &e) in s[idx_tenth + 1..].iter().enumerate() {
        if e == b'9' {
            idx_unit = idx_tenth + 1 + i;
            break;
        }

        if e > s[idx_unit] {
            idx_unit = idx_tenth + 1 + i;
        }
    }

    10 * ((s[idx_tenth] - b'0') as u32) + (s[idx_unit] - b'0') as u32
}

fn main() {
    let sum: u32 = aoc::lines()
        .filter(|l| !l.is_empty())
        .map(|l| process_line(l.as_bytes()))
        .sum();

    println!("Answer: {sum}");
}
