use aoc::input::{input, ParseLines};

aoc::test::test_part!(test1, part1, 14180628689);
aoc::test::test_part!(test2, part2, 1690);

const MODULUS: i64 = 16777216;
const EVOLUTIONS: usize = 2000;

fn evolve(mut secret: i64) -> i64 {
	secret = (secret ^ (64 * secret)) % MODULUS;
	secret = (secret ^ (secret / 32)) % MODULUS;
	secret = (secret ^ (2048 * secret)) % MODULUS;
	secret
}

pub fn part1() -> i64 {
	input!()
		.parse_lines()
		.map(|mut secret: i64| {
			for _ in 0..EVOLUTIONS {
				secret = evolve(secret);
			}
			secret
		})
		.sum()
}

struct PriceAndChange {
	price: i64,
	change: i64,
}

pub fn part2() -> i64 {
	let prices_and_changes: Vec<Vec<PriceAndChange>> = input!()
		.parse_lines()
		.map(|mut secret: i64| {
			let mut prices_and_changes = Vec::with_capacity(EVOLUTIONS);
			for _ in 0..EVOLUTIONS {
				let next = evolve(secret);
				prices_and_changes.push(PriceAndChange {
					price: next % 10,
					change: next % 10 - secret % 10,
				});
				secret = next;
			}
			prices_and_changes
		})
		.collect();
	let mut max_bananas = 0;
	for i in -9..=9 {
		for j in -9..=9 {
			for k in -9..=9 {
				for l in -9..=9 {
					let bananas = prices_and_changes
						.iter()
						.map(|monkey| {
							monkey
								.windows(4)
								.find(|window| {
									window
										.iter()
										.map(|pc| pc.change)
										.eq([i, j, k, l].into_iter())
								})
								.map_or(0, |window| window[3].price)
						})
						.sum();
					max_bananas = max_bananas.max(bananas);
				}
			}
		}
	}
	max_bananas
}
