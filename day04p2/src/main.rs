use aoc;

fn main() {
    // Return 1 if n is '@', 0 if '.'
    let normalize = |n| (n - b'.') / (b'@' - b'.');

    let mut input = aoc::lines()
        .filter(|l| !l.is_empty())
        .map(String::into_bytes)
        .collect::<Vec<_>>();
    input
        .iter_mut()
        .for_each(|line| line.iter_mut().for_each(|b| *b = normalize(*b)));

    let max_i = input.len();
    let max_j = input[0].len();

    let mut sum = 0;
    loop {
        let mut round = 0;
        for i in 0..max_j {
            for j in 0..max_j {
                if input[i][j] == 0 {
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
                        ballots +=
                            input[(i as isize + di) as usize][(j as isize + dj) as usize] as u32;
                    }
                }

                if ballots < 5 {
                    // println!("({i:2}, {j:2}) is free");
                    input[i][j] = 0;
                    round += 1;
                }
            }
        }

        if round == 0 {
            break;
        }
        sum += round;
    }

    println!("Answer: {sum}")
}
