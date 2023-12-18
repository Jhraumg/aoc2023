use itertools::Itertools;
use std::cmp::{max, min};
use std::str::Lines;

#[derive(Debug)]
struct Conversion {
    source: u64,
    dest: u64,
    len: u64,
}

#[derive(Debug, Default)]
struct Converter {
    conversions: Vec<Conversion>,
}

impl Converter {
    pub fn convert(&self, source: u64) -> u64 {
        for c in &self.conversions {
            if c.source <= source && source < (c.source + c.len) {
                let offset = source - c.source;
                return c.dest + offset;
            }
        }
        source
    }

    pub fn convert_range(&self, source: u64, len: u64) -> Vec<(u64, u64)> {
        let mut converted: Vec<(u64, u64)> = vec![];
        let mut to_convert: Vec<(u64, u64)> = vec![(source, len)];

        while !to_convert.is_empty() {
            let evaluated = to_convert.clone();
            to_convert.clear();
            for (source, len) in evaluated {
                let mut unmodified = Some((source, len));
                for conversion in &self.conversions {
                    if conversion.source >= source + len
                        || conversion.source + conversion.len <= source
                    {
                        // no intersect : range is not modified by this conversion
                        continue;
                    }
                    // here, there is an intersection
                    unmodified = None;
                    let intersect_source = max(conversion.source, source);
                    let intersect_len =
                        min(source + len, conversion.source + conversion.len) - intersect_source;
                    converted.push((self.convert(intersect_source), intersect_len)); // TODO : just compute offset and gain a loop !
                                                                                     // 0 1 2 3 4 5
                                                                                     //     2 3 4 <- 2,3
                                                                                     // 0 1 2 3 <- 0,4
                                                                                     // 2,2  reste 0,2
                    if intersect_source > source {
                        to_convert.push((source, intersect_source - source));
                    }

                    // 0 1 2 3 4 5
                    // 0 1 2 3 <- 0,4
                    //     2 3 4 <- 2,3
                    // 2,2  reste 4, 1
                    if intersect_source + intersect_len < source + len {
                        to_convert.push((
                            intersect_source + intersect_len,
                            source + len - intersect_source - intersect_len,
                        ));
                    }
                }
                if let Some(unmodified) = unmodified {
                    converted.push(unmodified)
                };
            }
        }

        converted
    }
}

fn read_converter(lines: &mut Lines) -> Converter {
    assert!(lines.next().unwrap().contains("map:"));
    let mut converter: Converter = Default::default();
    for l in lines.by_ref() {
        if l.is_empty() {
            break;
        }
        let (dest, source, len) =
            l.split(' ').filter_map(|w| w.parse::<u64>().ok()).collect_tuple().unwrap();
        converter.conversions.push(Conversion { source, dest, len });
    }
    converter
}

fn get_location(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds: Vec<u64> = lines
        .next()
        .and_then(|l| l.split_once(':'))
        .map(|(_, seeds)| seeds.split(' ').filter_map(|s| s.parse::<u64>().ok()).collect())
        .unwrap();
    let _ = lines.next();

    let seed_to_soil = read_converter(&mut lines);
    let soil_to_fertilizer = read_converter(&mut lines);
    let fertilizer_to_water = read_converter(&mut lines);
    let water_to_light = read_converter(&mut lines);
    let light_to_temperature = read_converter(&mut lines);
    let temperature_to_humidity = read_converter(&mut lines);
    let humidity_to_location = read_converter(&mut lines);

    seeds
        .iter()
        .map(|s| {
            let mut result = *s;
            for converter in [
                &seed_to_soil,
                &soil_to_fertilizer,
                &fertilizer_to_water,
                &water_to_light,
                &light_to_temperature,
                &temperature_to_humidity,
                &humidity_to_location,
            ] {
                result = converter.convert(result);
            }
            result
        })
        .min()
        .unwrap()
}

fn get_full_location(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut seeds: Vec<(u64, u64)> = vec![];
    let (_, l) = lines.next().unwrap().split_once(':').unwrap();
    let mut vals = l.trim().split(' ');

    while let Some(seed) = vals.next() {
        if let Some(len) = vals.next() {
            seeds.push((seed.parse().unwrap(), len.parse().unwrap()));
        }
    }
    let _ = lines.next();

    let seed_to_soil = read_converter(&mut lines);
    let soil_to_fertilizer = read_converter(&mut lines);
    let fertilizer_to_water = read_converter(&mut lines);
    let water_to_light = read_converter(&mut lines);
    let light_to_temperature = read_converter(&mut lines);
    let temperature_to_humidity = read_converter(&mut lines);
    let humidity_to_location = read_converter(&mut lines);

    let mut result = seeds;
    for converter in [
        &seed_to_soil,
        &soil_to_fertilizer,
        &fertilizer_to_water,
        &water_to_light,
        &light_to_temperature,
        &temperature_to_humidity,
        &humidity_to_location,
    ] {
        result = result
            .into_iter()
            .flat_map(|(s, l)| converter.convert_range(s, l).into_iter())
            .collect();
    }

    result.into_iter().map(|(s, _)| s).min().unwrap()
}

pub fn process_seed() {
    let input = include_str!("../resources/day5_fertilizers.txt");
    let location = get_location(input);
    println!("location {location}");

    let full_location = get_full_location(input);
    println!("full location {full_location}");
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn aoc_example_works() {
        let input = indoc! {"
            seeds: 79 14 55 13

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
            56 93 4
        "};
        assert_eq!(35, get_location(input));
        assert_eq!(46, get_full_location(input));
        //137516820
    }
}
