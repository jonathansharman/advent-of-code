use aoc::{graph::Digraph, input};

aoc::test::test_part!(test1, part1, 649);
aoc::test::test_part!(test2, part2, ?);

fn paths_to_out(
	network: &Digraph<&'static str>,
	source: &'static str,
) -> usize {
	if source == "out" {
		return 1;
	}
	match network.edges_from(source) {
		Some(sinks) => {
			sinks.keys().map(|&sink| paths_to_out(network, sink)).sum()
		}
		None => 0,
	}
}

pub fn part1() -> usize {
	let network: Digraph<&'static str> = input!()
		.lines()
		.flat_map(|line| {
			let (source, sinks) = line.split_once(": ").unwrap();
			sinks.split_whitespace().map(move |sink| (source, sink, 1))
		})
		.collect();
	paths_to_out(&network, "you")
}

pub fn part2() -> usize {
	0
}
