use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-3.exe FILE\n\tWhere PART is one of `1` or `2`");
        return;
    }
    let Ok(lines) = File::open(args[2].clone()).map(|f| BufReader::new(f).lines()) else {
        eprintln!("Error reading `{}`", args[2]);
        return;
    };
    let sum: u64 = match args[1].as_str() {
        "1" => part1(lines),
        "2" => part2(lines),
        _ => {
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-3.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got calibration! `{sum}`");
}

fn part1(mut lines: std::io::Lines<BufReader<File>>) -> u64 {
    let is_symbol = |c: u8| !c.is_ascii_digit() && c != b'.';
    let mut sum: u64 = 0;
    let mut current_lines = [Some(Ok(String::new())), lines.next(), lines.next()]
        .map(Option::unwrap)
        .map(Result::unwrap)
        .map(String::into_bytes);
    let width: usize = current_lines[1].len();
    current_lines[0] = ".".repeat(width).into_bytes();
    let mut lines = lines.chain([Ok(".".repeat(width)), Ok(".".repeat(width))]);
    while let Some(Ok(new_line)) = lines.next() {
        let mut parsing = false;
        let mut valid = false;
        let mut parsed = vec![];
        let line = &current_lines[1];
        let above = &current_lines[0];
        let below = &current_lines[2];
        for (x, b) in line.iter().enumerate() {
            let c = char::from_u32(u32::from(*b)).expect("Only ascii caracters");
            match (parsing, c.is_ascii_digit()) {
                (false, true) => {
                    parsing = true;
                    if is_symbol(above[x])
                        || is_symbol(below[x])
                        || (x > 0
                            && (is_symbol(line[x - 1])
                                || is_symbol(above[x - 1])
                                || is_symbol(below[x - 1])))
                    {
                        valid = true;
                    }
                    parsed.push(*b);
                }
                (true, true) => {
                    if is_symbol(above[x]) || is_symbol(below[x]) {
                        valid = true;
                    }
                    parsed.push(*b);
                }
                (true, false) => {
                    parsing = false;
                    if is_symbol(line[x]) || is_symbol(above[x]) || is_symbol(below[x]) {
                        valid = true;
                    }
                    if valid {
                        valid = false;
                        sum += parsed
                            .iter()
                            .map(|b| b - b'0')
                            .fold(0u64, |acc, d| acc * 10 + u64::from(d));
                    }
                    parsed.clear();
                }
                (false, false) => (),
            }
        }
        if parsing && valid {
            sum += parsed
                .iter()
                .map(|b| b - b'0')
                .fold(0u64, |acc, d| acc * 10 + u64::from(d));
        }
        current_lines.rotate_left(1);
        current_lines[2] = new_line.into_bytes();
    }

    sum
}

fn part2(mut lines: std::io::Lines<BufReader<File>>) -> u64 {
    let is_symbol = |c: u8, g: (usize, usize), m: &mut Vec<(usize, usize)>| {
        let symbol = !c.is_ascii_digit() && c != b'.';
        if c == b'*' {
            m.push(g);
        }
        symbol
    };
    let mut current_lines = [Some(Ok(String::new())), lines.next(), lines.next()]
        .map(Option::unwrap)
        .map(Result::unwrap)
        .map(String::into_bytes);
    let width: usize = current_lines[1].len();
    current_lines[0] = ".".repeat(width).into_bytes();
    let mut lines = lines.chain([Ok(".".repeat(width)), Ok(".".repeat(width))]);
    let mut y: usize = 0;
    let mut gears: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    while let Some(Ok(new_line)) = lines.next() {
        let mut parsing = false;
        let mut parsed_gears = vec![];
        let mut valid = false;
        let mut parsed = vec![];
        let line = &current_lines[1];
        let above = &current_lines[0];
        let below = &current_lines[2];
        for (x, b) in line.iter().enumerate() {
            let c = char::from_u32(u32::from(*b)).expect("Only ascii caracters");
            match (parsing, c.is_ascii_digit()) {
                (false, true) => {
                    parsing = true;
                    if is_symbol(above[x], (x, y.saturating_sub(1)), &mut parsed_gears)
                        || is_symbol(below[x], (x, y + 1), &mut parsed_gears)
                        || (x > 0
                            && (is_symbol(
                                line[x - 1],
                                (x.saturating_sub(1), y),
                                &mut parsed_gears,
                            ) || is_symbol(
                                above[x - 1],
                                (x.saturating_sub(1), y.saturating_sub(1)),
                                &mut parsed_gears,
                            ) || is_symbol(
                                below[x - 1],
                                (x.saturating_sub(1), y + 1),
                                &mut parsed_gears,
                            )))
                    {
                        valid = true;
                    }
                    parsed.push(*b);
                }
                (true, true) => {
                    if is_symbol(above[x], (x, y.saturating_sub(1)), &mut parsed_gears)
                        || is_symbol(below[x], (x, y + 1), &mut parsed_gears)
                    {
                        valid = true;
                    }
                    parsed.push(*b);
                }
                (true, false) => {
                    parsing = false;
                    if is_symbol(line[x], (x, y), &mut parsed_gears)
                        || is_symbol(above[x], (x, y.saturating_sub(1)), &mut parsed_gears)
                        || is_symbol(below[x], (x, y + 1), &mut parsed_gears)
                    {
                        valid = true;
                    }
                    if valid {
                        valid = false;
                        let value = parsed
                            .iter()
                            .map(|b| b - b'0')
                            .fold(0u64, |acc, d| acc * 10 + u64::from(d));
                        for gear in &parsed_gears {
                            gears
                                .entry(*gear)
                                .and_modify(|v| v.push(value))
                                .or_insert(vec![value]);
                        }
                        parsed_gears.clear();
                    }
                    parsed.clear();
                }
                (false, false) => (),
            }
        }
        if parsing && valid {
            let value = parsed
                .iter()
                .map(|b| b - b'0')
                .fold(0u64, |acc, d| acc * 10 + u64::from(d));
            for gear in &parsed_gears {
                gears
                    .entry(*gear)
                    .and_modify(|v| v.push(value))
                    .or_insert(vec![value]);
            }
            parsed_gears.clear();
        }
        y += 1;
        current_lines.rotate_left(1);
        current_lines[2] = new_line.into_bytes();
    }
    gears
        .values()
        .filter(|&v| (v.len() == 2)).map(|v| v[0] * v[1])
        .sum()
}
