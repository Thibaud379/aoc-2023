use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-4.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-4.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got calibration! `{sum}`")
}

fn part1(lines: std::io::Lines<BufReader<File>>) -> u64 {
    lines
        .map(|l| {
            let line = l.unwrap();
            let colon = line.find(':').expect("Input file to be well formed");
            let sets: Vec<HashSet<u64>> = line[colon + 2..]
                .split('|')
                .map(|list| {
                    list.split_ascii_whitespace()
                        .map(|e| e.parse().unwrap())
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>();
            let number_of_wins = sets[0].intersection(&sets[1]).count();
            match number_of_wins {
                0 => 0,
                _ => 2u64.pow((number_of_wins - 1) as u32),
            }
        })
        .sum()
}

fn part2(lines: std::io::Lines<BufReader<File>>) -> u64 {
    let mut cards = HashMap::new();
    lines.for_each(|l| {
        let line = l.unwrap();
        let colon = line.find(':').expect("Input file to be well formed");
        let card_id: usize = line[..colon]
            .split_ascii_whitespace()
            .skip(1)
            .next()
            .expect("Input file to be well formed")
            .parse()
            .unwrap();
        cards.entry(card_id).and_modify(|c| *c += 1).or_insert(1);
        let sets: Vec<HashSet<u64>> = line[colon + 2..]
            .split('|')
            .map(|list| {
                list.split_ascii_whitespace()
                    .map(|e| e.parse().unwrap())
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();
        let number_of_wins = sets[0].intersection(&sets[1]).count();
        for id in (card_id + 1)..(card_id + 1 + number_of_wins) {
            let card_ammount = *cards.get(&card_id).unwrap();
            cards
                .entry(id)
                .and_modify(|v| *v += card_ammount)
                .or_insert(card_ammount);
        }
    });
    println!("{cards:?}");
    cards.values().sum()
}
