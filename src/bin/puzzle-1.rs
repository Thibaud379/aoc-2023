use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-1.exe FILE\n\tWhere PART is one of `1` or `2`");
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
    lines
        .map(|line| {
            let (mut first, mut last) = (None, 0);
            for c in line.unwrap().chars().filter(char::is_ascii_digit) {
                let cv = c as u8 - '0' as u8;
                if first.is_none() {
                    first = Some(cv);
                }
                last = cv;
            }
            (10 * first.unwrap() + last) as u64
        })
        .sum()
}

fn part2(lines: std::io::Lines<BufReader<File>>) -> u64 {
    const STARTS: [(char, char); 9] = [
        ('o', 'n'),
        ('t', 'w'),
        ('t', 'h'),
        ('f', 'o'),
        ('f', 'i'),
        ('s', 'i'),
        ('s', 'e'),
        ('e', 'i'),
        ('n', 'i'),
    ];
    const FULLS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    lines
        .map(|line| {
            let line = line.unwrap();
            let mut chars = line.chars().enumerate(); // No read errors
            let mut c1 = chars.next().unwrap().1; // No empty lines
            let (mut first, mut last) = (None, 0);
            while let Some((idx, c2)) = chars.next() {
                if c1.is_ascii_digit() {
                    first.get_or_insert(c1 as u8 - '0' as u8);
                    last = c1 as u8 - '0' as u8;
                }
                if let Some(number) = STARTS.iter().position(|e| e.eq(&(c1, c2))) {
                    if line[(idx - 1)..].starts_with(FULLS[number]) {
                        first.get_or_insert(number as u8 + 1);
                        last = number as u8 + 1;
                    }
                }
                c1 = c2;
            }
            let last_char = line.chars().last().unwrap();
            if last_char.is_ascii_digit() {
                first.get_or_insert(last_char as u8 - '0' as u8);
                last = last_char as u8 - '0' as u8;
            }
            println!("{line} - {}{last}", first.unwrap());
            (10 * first.unwrap() + last) as u64
        })
        .sum()
}
