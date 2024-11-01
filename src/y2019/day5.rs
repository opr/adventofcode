use std::cmp::min;
use std::collections::{HashMap, HashSet};
use crate::input_reader::input_reader::read_input_for_day;

#[derive(Clone)]
struct LookupTable {
    source: usize,
    destination: usize,
    range: usize
}

pub fn task() {
    let mut input = read_input_for_day(2023, 5, true );
    let lines: Vec<_> = input.split("\n").collect();

    let (_, unparsed_seeds) = lines[0].split_once( "seeds: " ).unwrap();
    let seeds: Vec<_> = unparsed_seeds
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap() )
        .collect::<Vec<usize>>()
        .chunks(2).map( | chunk | {
        return (chunk[0], chunk[1]);
    }).collect();

    let map_names = [ "seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"];
    let mut parsed_maps = map_names.iter().map( | m | split_map(m, &input ) ).collect::<Vec<Vec<LookupTable>>>();


    let ( mut last, mut last_range ) = seeds[0];
    for map in &mut parsed_maps {
        for range in map {
            //let x = get_next_range_to_insert( (range.source, range.range), &map );
        }
    }
    find_from_range( seeds, &mut parsed_maps );
}

fn find_from_range( mut ranges: Vec<(usize, usize)>,  maps: &mut Vec<Vec<LookupTable>> ) -> usize {
    ranges = ranges.iter().map( | r | (r.0,r.0+r.1) ).collect();
    for map in maps {
        let mut new_range: Vec<(usize,usize)> = vec![];
        for range in &ranges {
            let converted_ranges = convert_ranges_with_map(range, map);
            new_range.append( &mut converted_ranges.clone() );
        }
        ranges = new_range;
    }

    let mut lowest = usize::MAX;
    for range in ranges {
        lowest = lowest.min( range.0 ).min( range.1 );
    }

    let f = 1;

    return 1;
}
/*
	private function find_lowest_location_from_ranges( array $ranges, array $maps ): ?int {
		foreach ( $maps as $map ) {
			$new_ranges = [];

			// Process each range through the current map by converting it to the new ranges and merging them.
			foreach ( $ranges as $range ) {
				$converted_ranges = $this->convert_range_using_mappings( $range, $map );
				$new_ranges       = array_merge( $new_ranges, $converted_ranges );
			}
			$ranges = $new_ranges;
		}

		$lowest_location = PHP_INT_MAX;
		foreach ( $ranges as $range ) {
			// Find the lowest number within all ranges.
			$lowest_location = min( $lowest_location, $range[0], $range[1] );
		}

		return $lowest_location === PHP_INT_MAX ? null : $lowest_location;
	}
 */



fn convert_ranges_with_map(range: &(usize, usize), map: &mut Vec<LookupTable> ) -> Vec<(usize,usize)> {
    let mut result: Vec<(usize,usize)> = vec![];
    let (min, max) = ( range.0, range.1);

    for mapping in map {
        let ( destination_start, source_start, range, source_end ) = ( mapping.destination, mapping.source, mapping.range, mapping.source + mapping.range );

        // Check if the current range overlaps with the mapping range.
        if min < source_end && max >= source_start {
            let mapped_min = min.max( source_start );
            let mapped_max = max.min( source_end - 1 );

            result.push( ( destination_start + ( mapped_min - source_start ), destination_start + ( mapped_max - source_start) ) );
        }
    }

    if result.len() == 0 {
        result.push( range.clone() );
    }

    return result;
}
/*
private function convert_range_using_mappings( array $range, array $mappings ): array {
		$result_ranges = [];
		list( $min, $max ) = $range;

		foreach ( $mappings as $mapping ) {
			list( $destination_start, $source_start, $length ) = $mapping;
			$source_end = $source_start + $length;

			// Check if the current range overlaps with the mapping range.
			if ( $min < $source_end && $max >= $source_start ) {
				// Find the overlap range and calculate the destination range.
				$mapped_min = max( $min, $source_start );
				$mapped_max = min( $max, $source_end - 1 );

				$result_ranges[] = [
					$destination_start + ( $mapped_min - $source_start ),
					$destination_start + ( $mapped_max - $source_start )
				];
			}
		}

		// If the range does not intersect with any mapping, add it unchanged.
		if ( empty( $result_ranges ) ) {
			$result_ranges[] = $range;
		}

		return $result_ranges;
	}
 */

fn get_next_range_to_insert( (seed_,seed_range_): (usize, usize), map: &mut Vec<LookupTable> ) -> Option<(usize, usize)> {
    let min = seed_;
    let max = seed_+seed_range_;

    let (mut seed, mut seed_range) = (seed_, seed_range_);

    let relevant_range = &map
        .iter()
        .find(|range| range.source <= seed && seed <= range.source + range.range || range.source <= seed + seed_range && seed + seed_range <= range.source + range.range);

    if let Some(r) = relevant_range {
        let fully_contained = seed >= r.source && seed + seed_range < r.source + r.range;
        if !fully_contained {
            let overlap_range = (r.source.min(seed), (r.source).min((seed + seed_range) - 1));
            map.push(
                LookupTable {
                    source: overlap_range.0,
                    destination: overlap_range.0,
                    range: overlap_range.1 - overlap_range.0
                });
            seed = overlap_range.0;
            seed_range = overlap_range.1- overlap_range.0;
            return Some(( seed, seed_range) );
        }
        return Some((r.source, r.range));
    }
    map.push(LookupTable {
        source: seed,
        destination: seed,
        range: seed_range
    });
    return None;
}

fn range_lookup((seed,seed_range): (usize, usize), maps: &Vec<Vec<LookupTable>>) -> (usize, usize) {
    let mut last_range = (seed,seed_range);
    for current in maps {
        let relevant_range: Vec<_> = current
            .iter()
            .filter(|range| range.source <= seed && seed <= range.source + range.range || range.source <= seed+seed_range && seed+seed_range <= range.source+range.range )
            .collect();
        let r = 3;
        // if let Some(r) = relevant_range {
        //     //let diff: isize = (r.destination as isize - r.source as isize);
        //     //last_range = (r.source, r.range);
        //     continue;
        // }
    }
    return last_range;
}
fn lookup(initial: &usize, maps: &Vec<Vec<LookupTable>>) -> usize {
    let mut last_lookup = *initial;
    for current in maps {
        let relevant_range = current
            .iter()
            .find( | lookup_table | (lookup_table.source..lookup_table.source+lookup_table.range).contains(&last_lookup) );
        if let Some(r) = relevant_range {
            let diff: isize = (r.destination as isize - r.source as isize);
            last_lookup = (last_lookup as isize + diff) as usize;
            continue;
        }
    }
    return last_lookup;
}
fn reverse_lookup(initial: &usize, maps: &Vec<Vec<LookupTable>>) -> usize {
    let mut last_lookup = *initial;
    let rev_maps = maps.iter().cloned().rev().collect::<Vec<Vec<LookupTable>>>();

    for current in rev_maps {
        let relevant_range = current
            .iter()
            .find( | lookup_table | (lookup_table.destination..lookup_table.destination+lookup_table.range).contains(&last_lookup) );
        if let Some(r) = relevant_range {
            let diff: isize = (r.source as isize - r.destination as isize);
            last_lookup = (last_lookup as isize + diff) as usize;
            continue;
        }
    }
    return last_lookup;
}

fn split_map( map_name: &str, input: &String ) -> ( Vec<LookupTable> ) {
    let full_map_name = &format!("\n\n{} map:\n", map_name);
    let (_, map_t) = input.split_once(full_map_name).unwrap();
    let (map, _) = map_t.split_once("\n\n").unwrap_or((map_t, ""));
    let map_entries: Vec<Vec<usize>> = map
        .split( "\n" )
        .collect::<Vec<&str>>()
        .iter()
        .map( |entry| entry.split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map( |entry| entry.parse::<usize>().unwrap() )
            .collect::<Vec<_>>() ).collect();    let b = 4;
    return map_entries.iter().map( | e | LookupTable { source: e[1],destination: e[0], range: e[2] } ).collect::<Vec<LookupTable>>();

}

// pub fn split_map( map_name: &str, input: &String ) -> ( HashMap<usize, usize> ) {
//     let full_map_name = &format!("\n\n{} map:\n", map_name);
//     let (_, map_t) = input.split_once( full_map_name ).unwrap();
//     let ( map, _) = map_t.split_once("\n\n" ).unwrap_or( ( map_t, "" ) );
//
//     let map_entries: Vec<Vec<usize>> = map
//         .split( "\n" )
//         .collect::<Vec<&str>>()
//         .iter()
//         .map( |entry| entry.split(" ")
//             .collect::<Vec<&str>>()
//             .iter()
//             .map( |entry| entry.parse::<usize>().unwrap() )
//             .collect::<Vec<_>>() ).collect();
//
//     let mut entries: HashMap<usize, usize> = HashMap::new();
//
//     for e in &map_entries {
//         let destination = e[0];
//         let source = e[1];
//         let range = e[2];
//
//         for range_pointer in 0..range {
//             //println!("mapping destination: {} to source {}", destination+range_pointer, source+range_pointer );
//             entries.insert( source+range_pointer, destination+range_pointer );
//         }
//     }
//     return entries;
// }