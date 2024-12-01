use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-2.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-1.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got calibration! `{sum}`")
}

fn part1(lines: std::io::Lines<BufReader<File>>) -> u64 {
    const COLORS: [&str; 3] = ["red", "green", "blue"];
    const LIMITS: [u64; 3] = [12, 13, 14];
    lines
        .filter_map(|line| {
            println!("{line:?}");
            let line = line.unwrap();
            let semi_idx = line.find(':').unwrap(); // Line is well formated
            let mut game_id: Option<u64> =
                Some(line[5..semi_idx].parse().expect("game id to be a number"));
            for pull in line[(semi_idx + 2)..].split("; ") {
                pull.split(", ").for_each(|color| {
                    let space = color.find(char::is_whitespace).unwrap();
                    let (n, c) = (color[..space].parse().unwrap(), &color[(space + 1)..]);
                    let color_id = COLORS.iter().position(|col| col.eq(&c)).unwrap();
                    if LIMITS[color_id] < n {
                        game_id = None;
                        return;
                    };
                });
            }
            game_id
        })
        .sum()
}

fn part2(lines: std::io::Lines<BufReader<File>>) -> u64 {
    const COLORS: [&str; 3] = ["red", "green", "blue"];
    lines
        .map(|line| {
            println!("{line:?}");
            let line = line.unwrap();
            let semi_idx = line.find(':').unwrap(); // Line is well formated
            let mut max_c = [0u64, 0u64, 0u64];
            for pull in line[(semi_idx + 2)..].split("; ") {
                pull.split(", ").for_each(|color| {
                    let space = color.find(char::is_whitespace).unwrap();
                    let (n, c) = (color[..space].parse().unwrap(), &color[(space + 1)..]);
                    let color_id = COLORS.iter().position(|col| col.eq(&c)).unwrap();
                    max_c[color_id] = max_c[color_id].max(n);
                });
            }
            max_c[0] * max_c[1] * max_c[2]
        })
        .sum()
}
