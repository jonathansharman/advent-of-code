use crate::io::read_lines;

use std::collections::HashMap;

pub fn part1() -> i64 {
	let mut fishes = read_lines("input/2021/6.txt")
		.next()
		.unwrap()
		.split(',')
		.map(&str::parse)
		.map(Result::unwrap)
		.collect::<Vec<i64>>();
	for _ in 0..80 {
		let mut new_fish = Vec::new();
		for fish in fishes.iter_mut() {
			*fish -= 1;
			if *fish == -1 {
				new_fish.push(8);
				*fish = 6;
			}
		}
		fishes.append(&mut new_fish);
	}
	fishes.len() as i64
}

pub fn part2() -> i64 {
	let mut cache: HashMap<(i64, i64), i64> = HashMap::new();

	read_lines("input/2021/6.txt")
		.next()
		.unwrap()
		.split(',')
		.map(|fish| fish.parse::<i64>().unwrap())
		.fold(0, |acc, timer| acc + get(&mut cache, timer, 256))
}

fn get(cache: &mut HashMap<(i64, i64), i64>, timer: i64, days: i64) -> i64 {
	if cache.contains_key(&(timer, days)) {
		return cache[&(timer, days)];
	}
	let v = if days == 0 {
		1
	} else if timer == 0 {
		get(cache, 6, days - 1) + get(cache, 8, days - 1)
	} else {
		get(cache, timer - 1, days - 1)
	};
	cache.insert((timer, days), v);
	v
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(379114, super::part1());
	}

	#[test]
	fn part2() {
		assert_eq!(1702631502303, super::part2());
	}
}
