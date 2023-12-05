use std::error::Error;
use aochelpers::get_daily_input;

#[derive(Debug,PartialEq)]
struct AlmanacEntry {
    destination: i64,
    source: i64,
    offset: i64
}

struct Almanac {
    seed_list: Vec<i64>,
    mappings: Vec<Vec<AlmanacEntry>>
}


fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(05,2023)?;
    let almanac = parse_almanac(&data);
    println!("Part 1: {}", part1(&almanac));
    println!("Part 2: {}", part2(&almanac));

    Ok(())
}
fn part1(almanac: &Almanac) -> i64 {
    
    almanac.seed_list.iter().map(|s| grow_seed(*s, almanac)).min().unwrap_or(0)
}

fn part2(almanac: &Almanac) -> i64 {
    // TODO: Refactor brute force
    let mut values = almanac.seed_list.iter();
    let mut result = i64::MAX;

    while let Some(start) = values.next() {
        let count = values.next().unwrap();
        result = result.min(grow_seeds(*start, *count, almanac));
    }
    result
}

fn grow_seeds(start:i64, range: i64, almanac: &Almanac) -> i64 {
    let mut result = i64::MAX;
    for i in start..start+range {
        result = result.min(grow_seed(i, almanac))
    }
    result
}

fn parse_almanac_entry(entry: &str) -> Vec<AlmanacEntry> {
    let mut lines = entry.split('\n');
    let mut almanac_entry = Vec::new();
    lines.next();
    for line in lines {
        let mut numbers = line.split(" ").filter_map(|x| x.parse::<i64>().ok());
        almanac_entry.push(AlmanacEntry{
            destination: numbers.next().unwrap(), 
            source: numbers.next().unwrap(), 
            offset: numbers.next().unwrap()})
    }
    almanac_entry
}

fn parse_almanac(almanac: &str) -> Almanac {
    let mut entries = almanac.split("\n\n");
    let seed_list = entries.next().unwrap().split(' ').filter_map(|x| x.parse::<i64>().ok()).collect::<Vec<_>>();
    let mappings = entries.map(parse_almanac_entry).collect::<Vec<_>>();
    Almanac{seed_list, mappings}
}

fn grow_seed(mut id: i64, almanac: &Almanac) -> i64{

    for mapping in almanac.mappings.iter() {
        for entry in mapping {
            if id >= entry.source && id < entry.source + entry.offset {
                id = (id - entry.source ) + entry.destination;
                break;
            }
        }
    }

    id
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[test]
    fn test_parser() {
        assert_eq!(parse_almanac_entry("seed-to-soil map:
50 98 2
52 50 48"), vec![AlmanacEntry{destination: 50, source: 98, offset: 2}, AlmanacEntry{destination: 52, source: 50, offset: 48}])
    }

    #[test]
    fn test_seed_79() {
        let almanac = parse_almanac(DATA);
        assert_eq!(grow_seed(79, &almanac), 82);
        assert_eq!(grow_seed(14, &almanac), 43);
        assert_eq!(grow_seed(55, &almanac), 86);
        assert_eq!(grow_seed(13, &almanac), 35);

    }

    #[test]
    fn test_part1() {
        let almanac = parse_almanac(DATA);
        assert_eq!(part1(&almanac), 35);
    }


    #[test]
    fn test_part2() {
        let almanac = parse_almanac(DATA);
        assert_eq!(part2(&almanac), 46);
    }
}