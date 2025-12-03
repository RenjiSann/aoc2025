use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn lines() -> impl Iterator<Item = String> {
    let input_file = std::env::args().skip(1).next().expect("missing input file");
    let file = File::open(&input_file).expect(format!("Cannot open file {input_file}").as_str());

    let br = BufReader::new(file);

    br.lines().filter_map(|l| {
        l.ok()
            .and_then(|l| if l.is_empty() { None } else { Some(l) })
    })
}
