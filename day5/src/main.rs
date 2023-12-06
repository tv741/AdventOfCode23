use common_lib::get_input_cached;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
struct MapEntry {
    src: usize,
    dst: usize,
    len: usize
}

impl MapEntry {
    fn map(&self, i: usize) -> Option<usize> {
        let offset = i as isize - self.src as isize;
        if i >= self.src && offset < self.len as isize {
            Some(self.dst + offset as usize)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    entries: Vec<MapEntry>
}

impl Map {
    fn new() -> Self {
        Self {entries: Vec::new()}
    }

    fn map(&self, i: usize) -> usize {
        self.entries.iter().find_map(|m| m.map(i)).unwrap_or(i)
    }
}

fn main() {
    let input = get_input_cached(5, false);
    let mut lines = input.lines();
    let seeds: Vec<usize> = lines.next().unwrap().split_whitespace().skip(1).map(|s| s.parse::<usize>().unwrap()).collect();

    let mut maps = Vec::<Map>::new();
    for line in lines {
        if line.is_empty() { continue }
        let parts: Vec<_> = line.split_whitespace().collect();

        if parts[1] == "map:" {
            maps.push(Map::new());
        } else {
            let nums: Vec<_> = parts.iter().map(|s| s.parse::<usize>().unwrap()).collect();
            maps.last_mut().unwrap().entries.push(MapEntry { src: nums[1], dst: nums[0], len: nums[2]});
        }
    }

    let mut items = seeds.clone();
    for map in maps.iter() {
        items = items.iter().map(|&s| map.map(s)).collect();
    }
    println!("Part One {}", items.iter().min().unwrap());


    let tuples: Vec<_> = seeds.iter().tuples::<(_,_)>().collect(); 
    let min = tuples.par_iter().map(|(&start, &len)| {
        let mut min = usize::MAX;
        for (n, seed) in (start..(start+len)).enumerate() {
            let mut item = seed;
            for map in maps.iter() {
                item = map.map(item);
            }

            let new_min = min.min(item);
            if new_min != min {
                println!("min: {new_min}");
                min = new_min;
            }

            if n % (1024*1024) == 0 {
                dbg!(n as f32 / len as f32);
            }
        }
        min
    }).min();
        
    println!("Part Two {}", min.unwrap());

}
