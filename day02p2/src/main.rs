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

fn is_invalid(n: N) -> bool {
    let nb_digits = nb_digits(n);
    if nb_digits == 1 {
        return false;
    }
    for div in divisors_under_root(nb_digits) {
        let pattern = n % ((10 as N).pow(div));

        if pattern == 0 {
            continue;
        }

        let multiple: u64 = (0..nb_digits / div)
            .map(|i| pattern * (10 as N).pow(i * div))
            .sum();

        if n % multiple == 0 {
            return true;
        }
    }

    false
}

fn next_invalid(n: N) -> N {
    let digits = nb_digits(n);
    if digits % 2 != 0 {
        return N::pow(10, digits) + N::pow(10, digits / 2);
    }
    let ten_pow = N::pow(10, digits / 2);

    let below = n % ten_pow;
    let above = n / ten_pow;

    match below.cmp(&above) {
        Ordering::Less => n - below + above,
        Ordering::Equal => n,
        Ordering::Greater => (above + 1) * (1 + ten_pow),
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

            for i in a..=b {
                if is_invalid(i) {
                    println!("new invalid: {i}, (in: {})", (a..=b).contains(&i));
                    sum += i;
                }
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
