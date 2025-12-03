use std::cmp::Ordering;

use aoc;

type N = u64;

fn nb_digits(mut n: N) -> u32 {
    if n == 0 {
        return 1;
    };
    let mut digits = 1;
    while n / 10 > 0 {
        digits += 1;
        n /= 10;
    }
    return digits;
}

fn next_invalid(n: N) -> N {
    let digits = nb_digits(n);
    if digits % 2 != 0 {
        return N::pow(10, digits ) + N::pow(10, digits / 2);
    }
    let ten_pow = N::pow(10, digits / 2);

    let below = n % ten_pow;
    let above = n / ten_pow;

    match below.cmp(&above) {
        Ordering::Less => n - below + above,
        Ordering::Equal => n,
        Ordering::Greater => below + below * ten_pow,
    }
}

fn main() {
    let mut sum = 0;

    for line in aoc::lines() {
        for id_range in line.split(",") {
            if id_range.is_empty() {
                continue;
            }

            let [a, b] = id_range.split("-").collect::<Vec<_>>()[..] else {
                panic!()
            };
            let (a, b) = (a.parse::<N>().unwrap(), b.parse::<N>().unwrap());
            println!("Checking {a}-{b}");

            let mut i = a;

            while i <= b {
                let inv = next_invalid(i);
                println!("new invalid: {inv}, (in: {})", (a..=b).contains(&inv));
                if inv <= b {
                    sum += inv;
                }
                i = inv + 1
            }
        }
    }

    println!("Answer: {sum}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(0, 1)]
    #[case(1, 1)]
    #[case(3, 1)]
    #[case(5, 1)]
    #[case(10, 2)]
    #[case(111, 3)]
    fn test_nbdigits(#[case] nb: N, #[case] expected: u32) {
        assert_eq!(nb_digits(nb), expected);
    }
}
