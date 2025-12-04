use aoc;

fn main() {
    let input = aoc::lines().map(String::into_bytes).collect::<Vec<_>>();

    // Return 1 if n is '@', 0 if '.'
    let one = |n| ((n - b'.') / (b'@' - b'.')) as u32;

    let max_i = input.len();
    let max_j = input[0].len();

    let mut sum = 0;
    for i in 0..max_j {
        for j in 0..max_j {
            if input[i][j] != b'@' {
                continue;
            }

            let mut ballots = 0;

            let is: &[isize] = match i {
                0 => &[0, 1],
                x if x == max_i - 1 => &[-1, 0],
                _ => &[-1, 0, 1],
            };
            let js: &[isize] = match j {
                0 => &[0, 1],
                x if x == max_j - 1 => &[-1, 0],
                _ => &[-1, 0, 1],
            };

            for &di in is {
                for &dj in js {
                    let xx = one(input[(i as isize + di) as usize][(j as isize + dj) as usize]);
                    ballots += xx;
                }
            }

            if ballots < 5 {
                println!("({i:2}, {j:2}) is free");
                sum += 1;
            }
        }
    }

    println!("Answer: {sum}")
}
