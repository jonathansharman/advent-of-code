aoc::test::test_part!(test1, part1, 138);
aoc::test::test_part!(test2, part2, 226845233210288);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> usize {
	let mut lines = INPUT.lines();
	let earliest: usize = lines.next().unwrap().parse().unwrap();
	let (wait, id) = lines
		.next()
		.unwrap()
		.split(',')
		.filter_map(|s| {
			s.parse::<usize>().ok().map(|id| (id - earliest % id, id))
		})
		.min_by(|a, b| a.0.cmp(&b.0))
		.unwrap();
	wait * id
}

// All the inputs "happen" to be prime and are thus pairwise coprime, so we can
// use the Chinese remainder theorem (CRT).
pub fn part2() -> usize {
	let (divisors, remainders): (Vec<_>, Vec<_>) = INPUT
		.lines()
		.nth(1)
		.unwrap()
		.split(',')
		.enumerate()
		.filter_map(|(i, s)| {
			s.parse::<usize>().ok().map(|id| (id, (id - i % id) % id))
		})
		.unzip();
	crt(&divisors, &remainders)
}

fn crt(divisors: &[usize], remainders: &[usize]) -> usize {
	let product: usize = divisors.iter().product();
	let partial_products =
		divisors.iter().map(|d| product / d).collect::<Vec<_>>();
	let inverses = divisors
		.iter()
		.zip(partial_products.iter())
		.map(|(d, pp)| inverse(*pp, *d));
	remainders
		.iter()
		.zip(partial_products.iter())
		.zip(inverses)
		.map(|((a, b), c)| a * b * c)
		.sum::<usize>()
		% product
}

fn inverse(n: usize, modulo: usize) -> usize {
	let (n, modulo) = (n as i64, modulo as i64);

	let mut t = 0;
	let mut next_t = 1;
	let mut r = modulo;
	let mut next_r = n;

	while next_r != 0 {
		let quotient = r / next_r;
		(t, next_t) = (next_t, t - quotient * next_t);
		(r, next_r) = (next_r, r - quotient * next_r);
	}

	if t < 0 {
		(t + modulo) as usize
	} else {
		t as usize
	}
}
