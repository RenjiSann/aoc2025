use aoc;

const N: usize = 12;

fn process_line(s: String) -> u64 {
    let s = s.as_bytes();
    assert!(s.len() >= N);
    let s = s.iter().map(|c| (c - b'0') as u64).collect::<Vec<_>>();
    let mut rest = &s[..];

    let mut res = 0;

    for i in 0..N {
        // Looking for the ith digit of the number. Search the maximum digit in
        // the input vector while keeping enough space at the end of the vec to
        // account for the rest of the digits.
        //
        // Searching 3rd digit -> in the whole array EXCEPT the 9 last elements
        // (because we still have to look for 9 digits after).

        // i = 0 => N - i - 1 = 12 - 1 = 11
        // [..(len - 11)]
        let upper_bound = rest.len() - (N - i - 1);
        assert!(upper_bound <= rest.len());
        assert_eq!(rest[upper_bound..].len(), N - i - 1, );

        let mut max_idx = 0;
        let mut j = 1;
        while j < upper_bound {
            if rest[j] > rest[max_idx] {
                max_idx = j;
            }
            j += 1;
        }
        res *= 10;
        res += rest[max_idx];
        rest = &rest[max_idx + 1..];
    }

    println!("line output: {res}");
    res
}

fn main() {
    let sum: u64 = aoc::lines()
        .filter(|l| !l.is_empty())
        .map(process_line)
        .sum();

    println!("Answer: {sum}");
}
