use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Result<T> = std::result::Result<T, String>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn group_with_most_calories(lines: io::Lines<io::BufReader<File>>) -> Result<usize> {
    lines
        .group_by(|line| line.as_ref().map(|str| str.is_empty()).unwrap_or_default())
        .into_iter()
        .map(|(_, group)| group.map(|r| r.and_then(|s| s.parse::<usize>())));
    todo!();
}

fn main() {
    // File hosts must exist in current path before this produces output
    let lines = read_lines("day_1/input.txt").expect("input file should be readable");
    group_with_most_calories(lines).unwrap();
}
