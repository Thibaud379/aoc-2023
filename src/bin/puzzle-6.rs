use std::{
    collections::HashMap,
    convert::identity,
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-6.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-6.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}

fn part1(mut lines: std::io::Lines<BufReader<File>>) -> u64 {
    let compute_roots = |t: f64, r: f64| {
        println!("{t}, {r}");
        let d = (t * t - 4f64 * r).sqrt();
        let x1 = (t - d) / 2f64;
        let x2 = x1 + d;
        println!("{d}, {x1}, {x2}");
        (x1, x2)
    };
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let records: Vec<u64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    println!("{times:?}\n{records:?}");
    let mut prod: u64 = 1;
    for (t, r) in times.into_iter().zip(records.into_iter()) {
        let roots = compute_roots(t as f64, r as f64);
        let mut roots_int = (roots.0.ceil(), roots.1.floor());
        if roots.0 == roots_int.0 {
            roots_int.0 += 1f64;
        }
        if roots.1 == roots_int.1 {
            roots_int.1 -= 1f64;
        }
        let diff = (roots_int.1 - roots_int.0) as u64 + 1;
        println!("{roots:?} -> {diff}");
        prod *= diff;
    }
    prod
}

fn part2(mut lines: std::io::Lines<BufReader<File>>) -> u64 {
    let compute_roots = |t: f64, r: f64| {
        println!("{t}, {r}");
        let d = (t * t - 4f64 * r).sqrt();
        let x1 = (t - d) / 2f64;
        let x2 = x1 + d;
        println!("{d}, {x1}, {x2}");
        (x1, x2)
    };
    let time: u64 = lines
        .next()
        .unwrap()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();
    let record: u64 = lines
        .next()
        .unwrap()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();
    println!("{time:?}\n{record:?}");

    let roots = compute_roots(time as f64, record as f64);
    let mut roots_int = (roots.0.ceil(), roots.1.floor());
    if roots.0 == roots_int.0 {
        roots_int.0 += 1f64;
    }
    if roots.1 == roots_int.1 {
        roots_int.1 -= 1f64;
    }
    let diff = (roots_int.1 - roots_int.0) as u64 + 1;
    println!("{roots_int:?} -> {diff}");
    diff
}
