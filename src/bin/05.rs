advent_of_code::solution!(5);

#[derive(Debug)]
struct Map {
    dest: u64,
    source: u64,
    range: u64,
}

impl Map {
    pub fn resolve(&self, from: u64) -> Option<u64> {
        println!("Resolving {} using {:?}", from, self);
        /* If from is part of the source range, resolve it to a destination */
        if self.source <= from && from < self.source + self.range {
            Some(self.dest + from - self.source)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,

    maps: Vec<Vec<Map>>,
    // seed_to_soil: Vec<Map>,
    // soil_to_fertilizer: Vec<Map>,
    // fertilizer_to_water: Vec<Map>,
    // water_to_light: Vec<Map>,
    // light_to_temperature: Vec<Map>,
    // temperature_to_humidity: Vec<Map>,
    // humidity_to_location: Vec<Map>,
}

fn parse_seeds_part_1(input: &str) -> Vec<u64> {
    /* Parse seeds as numbers */
    let (_, nums) = input.split_once(':').unwrap();

    nums.trim()
        .split(' ')
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

fn parse_seeds_part_2(input: &str) -> Vec<u64> {
    /* Parse seeds as ranges */
    let (_, nums) = input.split_once(':').unwrap();

    let nums: Vec<u64> = nums
        .trim()
        .split(' ')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    /* TODO: continue parsing this: */
    // nums.chunks(2).map(|(start, range)| )

    Vec::new()
}

fn parse_nums(input: &str) -> Map {
    let mut x = input.trim().split(' ').map(|n| n.trim().parse().unwrap());

    Map {
        dest: x.next().unwrap(),
        source: x.next().unwrap(),
        range: x.next().unwrap(),
    }
}

fn parse_input(input: &str, part: u32) -> Almanac {
    /* First line is seeds */
    let mut lines = input.lines();
    let fl = lines.next().unwrap();
    let seeds = if part == 1 {
        parse_seeds_part_1(fl)
    } else {
        parse_seeds_part_2(fl)
    };

    /* The rest are maps */
    let mut seed_to_soil: Vec<Map> = Vec::new();
    let mut soil_to_fertilizer: Vec<Map> = Vec::new();
    let mut fertilizer_to_water: Vec<Map> = Vec::new();
    let mut water_to_light: Vec<Map> = Vec::new();
    let mut light_to_temperature: Vec<Map> = Vec::new();
    let mut temperature_to_humidity: Vec<Map> = Vec::new();
    let mut humidity_to_location: Vec<Map> = Vec::new();

    let mut current_map: &mut Vec<Map> = &mut seed_to_soil;
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        match line.trim() {
            "seed-to-soil map:" => current_map = &mut seed_to_soil,
            "soil-to-fertilizer map:" => current_map = &mut soil_to_fertilizer,
            "fertilizer-to-water map:" => current_map = &mut fertilizer_to_water,
            "water-to-light map:" => current_map = &mut water_to_light,
            "light-to-temperature map:" => current_map = &mut light_to_temperature,
            "temperature-to-humidity map:" => current_map = &mut temperature_to_humidity,
            "humidity-to-location map:" => current_map = &mut humidity_to_location,

            _ => current_map.push(parse_nums(line)),
        }
    }

    Almanac {
        seeds,
        maps: vec![
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ],
        // seed_to_soil,
        // soil_to_fertilizer,
        // fertilizer_to_water,
        // water_to_light,
        // light_to_temperature,
        // temperature_to_humidity,
        // humidity_to_location,
    }
}

fn map_one_step(from: u64, maps: &[Map]) -> u64 {
    if let Some(dest) = maps.iter().find_map(|m| m.resolve(from)) {
        dest
    } else {
        from
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let alm = parse_input(input, 1);
    println!("{:?}", alm);

    let mut locations: Vec<u64> = Vec::with_capacity(alm.maps.len());

    for seed in alm.seeds {
        let mut x = seed;
        for maps in alm.maps.iter() {
            x = map_one_step(x, maps);
        }
        locations.push(x);
    }

    Some(*locations.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let alm = parse_input(input, 2);
    println!("{:?}", alm);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
