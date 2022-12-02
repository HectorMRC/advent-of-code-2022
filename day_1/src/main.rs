use itertools::Itertools;

fn parse_input(line_r: Result<String, std::io::Error>) -> Result<Option<usize>, String> {
    line_r
        .map_err(|err| err.to_string())
        .map(|line| line.parse::<usize>().ok())
}

fn main() {
    let mut slots = 0_usize;
    let slot_index = |line: &Option<usize>| -> usize {
        if line.is_none() {
            slots += 1;
        }

        slots
    };

    let file = util::read_lines("day_1/input.txt").expect("read input file");

    let lines: Vec<usize> = file
        .into_iter()
        .map(parse_input)
        .collect::<Result<Vec<Option<usize>>, String>>()
        .expect("read all file lines")
        .into_iter()
        .group_by(slot_index)
        .into_iter()
        .map(|(_, group)| group.map(|item| item.unwrap_or_default()).sum::<usize>())
        .sorted()
        .collect();

    println!("max amount of calories: {}", lines[slots]);
    println!(
        "sum of amount of calories: {}",
        lines.iter().rev().take(3).sum::<usize>()
    );
}
