#![feature(yield_expr)]
#![feature(gen_blocks)]
use std::cmp::Ordering;

use aoc;

type N = u64;

fn nb_digits(n: N) -> u32 {
    if n == 0 {
        return 1;
    }
    (n as f64).log10().floor() as u32 + 1
}

fn divisors_under_root(n: u32) -> impl Iterator<Item = u32> {
    gen move {
        for i in 1..=(n as f64).sqrt().floor() as u32 {
            if n % i == 0 {
                yield i;
                let ni = n / i;
                if ni != i && ni != n {
                    yield ni;
                }
            }
        }
    }
}

fn next_invalid(n: N) -> N {
    if n < 10 {
        return 11;
    }

    let digits = nb_digits(n);
    let mut lowest = N::MAX;

    let pow10 = |x| (10 as N).pow(x);

    for div in divisors_under_root(digits)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        // retrieve highest part of the digit
        let mut high = n / pow10(digits - div);

        let mut repeated: u64 = (0..digits / div).map(|i| high * pow10(i * div)).sum();

        // check that high_high_high_high.... is higher
        loop {
            if repeated >= n {
                break;
            }
            high += 1;
            repeated = (0..digits / div).map(|i| high * pow10(i * div)).sum();
        }

        lowest = lowest.min(repeated);
    }

    lowest
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
                if inv <= b {
                    println!("new invalid: {inv}, (in: {})", (a..=b).contains(&inv));
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
    use super::*;
    use rstest::rstest;

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
