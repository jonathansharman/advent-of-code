use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 32976912643);
aoc::test::test_part!(test2, part2, 54446379122);

fn ranges() -> impl Iterator<Item = (usize, usize)> {
	input!().split(',').map(|range| {
		range
			.split('-')
			.map(|s| s.parse::<usize>().unwrap())
			.collect_tuple()
			.unwrap()
	})
}

pub fn part1() -> usize {
	ranges()
		.flat_map(|(min, max)| {
			(min..=max).filter(|id| {
				let id = id.to_string();
				id[..id.len() / 2] == id[id.len() / 2..]
			})
		})
		.sum()
}

pub fn part2() -> usize {
	ranges()
		.flat_map(|(min, max)| {
			(min..=max).filter(|id| {
				let id = id.to_string();
				'chunks: for chunk_size in 1..=id.len() / 2 {
					let chunks: Vec<String> = id
						.chars()
						.chunks(chunk_size)
						.into_iter()
						.map(|chunk| chunk.collect())
						.collect();
					for (i, chunk1) in chunks.iter().enumerate() {
						for chunk2 in &chunks[i..] {
							if chunk1 != chunk2 {
								continue 'chunks;
							}
						}
					}
					return true;
				}
				false
			})
		})
		.sum()
}
