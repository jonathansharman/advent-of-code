use std::collections::HashMap;

use aoc::{graph::Digraph, input};

aoc::test::test_part!(test1, part1, 649);
aoc::test::test_part!(test2, part2, ?);

fn parse_network() -> Digraph<&'static str> {
	input!()
		.lines()
		.flat_map(|line| {
			let (source, sinks) = line.split_once(": ").unwrap();
			sinks.split_whitespace().map(move |sink| (source, sink, 1))
		})
		.collect()
}

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
	let network = parse_network();
	paths_to_out(&network, "you")
}

fn paths_through_dac_and_fft(
	cache: &mut HashMap<(&'static str, bool, bool), usize>,
	network: &Digraph<&'static str>,
	source: &'static str,
	mut dac: bool,
	mut fft: bool,
) -> usize {
	if let Some(&paths) = cache.get(&(source, dac, fft)) {
		return paths;
	}

	if source == "out" {
		return if dac && fft { 1 } else { 0 };
	}
	dac |= source == "dac";
	fft |= source == "fft";
	let paths = match network.edges_from(source) {
		Some(sinks) => sinks
			.keys()
			.map(|&sink| {
				paths_through_dac_and_fft(cache, network, sink, dac, fft)
			})
			.sum(),
		None => 0,
	};
	cache.insert((source, dac, fft), paths);
	paths
}

pub fn part2() -> usize {
	let network = parse_network();
	paths_through_dac_and_fft(
		&mut HashMap::new(),
		&network,
		"svr",
		false,
		false,
	)
}
