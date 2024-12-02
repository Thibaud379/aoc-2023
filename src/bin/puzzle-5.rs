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
        eprintln!("Not enough arguments\nUSAGE: PART ./puzzle-5.exe FILE\n\tWhere PART is one of `1` or `2`");
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
            eprint!("Arguments invalid\nUSAGE: PART ./puzzle-5.exe FILE\n\tWhere PART must be one of `1` or `2`");
            return;
        }
    };
    println!("Got result `{sum}`!");
}
#[derive(Clone, Debug)]
struct RangeMap {
    src: usize,
    dest: usize,
    size: usize,
}

impl RangeMap {
    fn contains(&self, value: usize) -> bool {
        (self.src..(self.src + self.size)).contains(&value)
    }

    fn map(&self, value: usize) -> usize {
        let v = if self.contains(value) {
            (value as i128 + (self.dest as i128 - self.src as i128)) as usize
        } else {
            value
        };
        v
    }

    fn from_line(line: &str) -> Self {
        let vals: Vec<_> = line
            .split_ascii_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        Self {
            dest: vals[0],
            src: vals[1],
            size: vals[2],
        }
    }
}

fn map_value(maps: &Vec<RangeMap>, value: usize) -> usize {
    // Assume maps is sorted in ascending src value
    let map_id = maps
        .binary_search_by(|e| e.src.cmp(&value))
        .map_or_else(|id| id.saturating_sub(1), identity);
    maps[map_id].map(value)
}
#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum FilePart {
    SeedSoil,
    SoilFert,
    FertWater,
    WaterLight,
    LightTemp,
    TempHum,
    HumLoc,
}

impl FilePart {
    fn try_from_header(header: &str) -> Result<Self, String> {
        match header {
            "seed-to-soil map" => Ok(Self::SeedSoil),
            "soil-to-fertilizer map" => Ok(Self::SoilFert),
            "fertilizer-to-water map" => Ok(Self::FertWater),
            "water-to-light map" => Ok(Self::WaterLight),
            "light-to-temperature map" => Ok(Self::LightTemp),
            "temperature-to-humidity map" => Ok(Self::TempHum),
            "humidity-to-location map" => Ok(Self::HumLoc),
            _ => Err(header.to_string()),
        }
    }
}
fn part1(mut lines: std::io::Lines<BufReader<File>>) -> u64 {
    let mut maps: HashMap<FilePart, Vec<RangeMap>> = HashMap::new();
    let first_line = lines.next().unwrap().unwrap();
    let seeds: Vec<usize> = first_line[(first_line.find(':').unwrap() + 2)..]
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let mut curr_part = FilePart::SeedSoil;
    while let Some(Ok(line)) = lines.next() {
        if let Some(colon) = line.find(':') {
            curr_part = FilePart::try_from_header(&line[..colon]).expect("Fil is well formed");
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let range = RangeMap::from_line(&line);
        maps.entry(curr_part)
            .and_modify(|v| v.push(range.clone()))
            .or_insert(vec![range]);
    }
    maps.values_mut()
        .for_each(|v| v.sort_by(|a, b| a.src.cmp(&b.src)));
    let locs: Vec<usize> = seeds
        .into_iter()
        .map(|s| map_value(&maps[&FilePart::SeedSoil], s))
        .map(|s| map_value(&maps[&FilePart::SoilFert], s))
        .map(|s| map_value(&maps[&FilePart::FertWater], s))
        .map(|s| map_value(&maps[&FilePart::WaterLight], s))
        .map(|s| map_value(&maps[&FilePart::LightTemp], s))
        .map(|s| map_value(&maps[&FilePart::TempHum], s))
        .map(|s| map_value(&maps[&FilePart::HumLoc], s))
        .collect();

    println!("{locs:?}");
    let min_loc = locs.into_iter().min().unwrap();
    min_loc as u64
}
fn map_range(maps: &Vec<RangeMap>, range: Range<usize>) -> Vec<Range<usize>> {
    // Assume maps is sorted in ascending src value
    let start_id = maps
        .binary_search_by(|e| e.src.cmp(&range.start))
        .map_or_else(|id| id.saturating_sub(1), identity);
    let end_id = maps
        .binary_search_by(|e| e.src.cmp(&range.end))
        .map_or_else(|id| id.saturating_sub(1), identity);
    if start_id == end_id {
        vec![maps[start_id].map(range.start)..(maps[start_id].map(range.end - 1) + 1)]
    } else {
        let mut res =
            vec![maps[start_id].map(range.start)..(maps[start_id].dest + maps[start_id].size)];
        for map_range in &maps[(start_id + 1)..end_id] {
            res.push(map_range.dest..(map_range.dest + map_range.size));
        }
        res.push((maps[end_id].dest)..(maps[end_id].map(range.end - 1) + 1));
        res
    }
}
fn merge_ranges<T: Iterator<Item = Range<usize>>>(ranges: T) -> Vec<Range<usize>> {
    let mut ranges = ranges.collect::<Vec<_>>();
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut range = ranges[0].clone();
    let mut res = vec![];
    for r in &ranges[1..] {
        if r.start <= range.end {
            range.end = r.end;
        } else {
            res.push(range);
            range = r.clone();
        }
    }
    res.push(range);
    res
}

fn part2(mut lines: std::io::Lines<BufReader<File>>) -> u64 {
    let mut maps: HashMap<FilePart, Vec<RangeMap>> = HashMap::new();
    let first_line = lines.next().unwrap().unwrap();
    let seeds_data: Vec<usize> = first_line[(first_line.find(':').unwrap() + 2)..]
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let mut seed_ranges = vec![];
    for i in 0..seeds_data.len() / 2 {
        let seed = seeds_data[2 * i];
        let range = seeds_data[2 * i + 1];
        seed_ranges.push(seed..(seed + range));
    }
    let mut curr_part = FilePart::SeedSoil;
    while let Some(Ok(line)) = lines.next() {
        if let Some(colon) = line.find(':') {
            curr_part = FilePart::try_from_header(&line[..colon]).expect("Fil is well formed");
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let range = RangeMap::from_line(&line);
        maps.entry(curr_part)
            .and_modify(|v| v.push(range.clone()))
            .or_insert(vec![range]);
    }
    maps.values_mut()
        .for_each(|v| v.sort_by(|a, b| a.src.cmp(&b.src)));
    let locs: Vec<Range<usize>> = seed_ranges
        .into_iter()
        .map(|s| merge_ranges(map_range(&maps[&FilePart::SeedSoil], s).into_iter()))
        .map(|s| {
            merge_ranges(
                s.into_iter()
                    .flat_map(|r| map_range(&maps[&FilePart::SoilFert], r)),
            )
        })
        .map(|s| {
            merge_ranges(
                s.into_iter()
                    .flat_map(|r| map_range(&maps[&FilePart::FertWater], r)),
            )
        })
        .map(|s| {
            merge_ranges(
                s.into_iter()
                    .flat_map(|r| map_range(&maps[&FilePart::WaterLight], r)),
            )
        })
        .map(|s| {
            merge_ranges(
                s.into_iter()
                    .flat_map(|r| map_range(&maps[&FilePart::LightTemp], r)),
            )
        })
        .map(|s| {
            merge_ranges(
                s.into_iter()
                    .flat_map(|r| map_range(&maps[&FilePart::TempHum], r)),
            )
        })
        .flat_map(|s| {
            merge_ranges(
                s.into_iter()
                    .flat_map(|r| map_range(&maps[&FilePart::HumLoc], r)),
            )
        })
        .collect();

    // let r = map_range(&maps[&FilePart::SeedSoil], 79..(79 + 14));
    // println!("{:?}", r);
    // let r = merge_ranges(
    //     r.into_iter()
    //         .flat_map(|ra| map_range(&maps[&FilePart::SoilFert], ra)),
    // );
    // println!("{:?}", r);
    // let r = merge_ranges(
    //     r.into_iter()
    //         .flat_map(|ra| map_range(&maps[&FilePart::FertWater], ra)),
    // );
    // println!("{:?}", r);
    // let r = merge_ranges(
    //     r.into_iter()
    //         .flat_map(|ra| map_range(&maps[&FilePart::WaterLight], ra)),
    // );
    // println!("{:?}", r);
    // let r = merge_ranges(
    //     r.into_iter()
    //         .flat_map(|ra| map_range(&maps[&FilePart::LightTemp], ra)),
    // );
    // println!("{:?}", r);
    // let r = merge_ranges(
    //     r.into_iter()
    //         .flat_map(|ra| map_range(&maps[&FilePart::TempHum], ra)),
    // );
    // println!("{:?}", r);
    // let r = merge_ranges(
    //     r.into_iter()
    //         .flat_map(|ra| map_range(&maps[&FilePart::HumLoc], ra)),
    // );
    // println!("{:?}", r);

    println!("{locs:?}");
    let min_loc = locs
        .into_iter()
        .min_by(|a, b| a.start.cmp(&b.start))
        .unwrap();
    min_loc.start as u64
}
