#[derive(PartialEq, Eq, Clone, strum_macros::Display)]
pub enum GameInput {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for GameInput {
    fn from(s: &str) -> Self {
        if s == "A" || s == "X" {
            Self::Rock
        } else if s == "B" || s == "Y" {
            Self::Paper
        } else {
            Self::Scissors
        }
    }
}

impl From<&GameInput> for usize {
    fn from(input: &GameInput) -> Self {
        match input {
            GameInput::Rock => 1_usize,
            GameInput::Paper => 2_usize,
            GameInput::Scissors => 3_usize,
        }
    }
}

fn parse_input(line_r: Result<String, std::io::Error>) -> Result<(GameInput, GameInput), String> {
    line_r.map_err(|err| err.to_string()).and_then(
        |line: String| -> Result<(GameInput, GameInput), String> {
            let mut it = line.split(" ");
            let (Some(first), Some(second)) = (it.next(), it.next()) else {
                return Err("invalid input".to_string());
            };

            Ok((first.into(), second.into()))
        },
    )
}

fn compute_points(
    mut points: (usize, usize),
    (first, second): (GameInput, GameInput),
) -> (usize, usize) {
    points.0 += usize::from(&first);
    points.1 += usize::from(&second);

    if first == second {
        points.0 += 3_usize;
        points.1 += 3_usize;
    } else if first == GameInput::Rock && second == GameInput::Paper
        || first == GameInput::Paper && second == GameInput::Scissors
        || first == GameInput::Scissors && second == GameInput::Rock
    {
        points.1 += 6_usize;
    } else {
        points.0 += 6_usize;
    }

    (points.0, points.1)
}

fn fist_strategy_score(input: Vec<(GameInput, GameInput)>) -> (usize, usize) {
    input.into_iter().fold((0_usize, 0_usize), compute_points)
}

fn second_strategy_score(input: Vec<(GameInput, GameInput)>) -> (usize, usize) {
    input
        .into_iter()
        .map(|(oponent, command)| match oponent {
            GameInput::Rock => (
                oponent,
                match command {
                    GameInput::Rock => GameInput::Scissors,
                    GameInput::Paper => GameInput::Rock,
                    GameInput::Scissors => GameInput::Paper,
                },
            ),
            GameInput::Paper => (
                oponent,
                match command {
                    GameInput::Rock => GameInput::Rock,
                    GameInput::Paper => GameInput::Paper,
                    GameInput::Scissors => GameInput::Scissors,
                },
            ),
            GameInput::Scissors => (
                oponent,
                match command {
                    GameInput::Rock => GameInput::Paper,
                    GameInput::Paper => GameInput::Scissors,
                    GameInput::Scissors => GameInput::Rock,
                },
            ),
        })
        .fold((0_usize, 0_usize), compute_points)
}

fn main() {
    let file = util::read_lines("day_2/input.txt").expect("read input file");

    let lines = file
        .into_iter()
        .map(parse_input)
        .collect::<Result<Vec<(GameInput, GameInput)>, String>>()
        .expect("parse all lines");

    let (first, second) = fist_strategy_score(lines.clone());
    println!("on first strategy...");
    println!("-\tfirst player got {}", first);
    println!("-\tsecond player got {}", second);

    let (first, second) = second_strategy_score(lines);
    println!("on second strategy...");
    println!("-\tfirst player got {}", first);
    println!("-\tsecond player got {}", second);
}
