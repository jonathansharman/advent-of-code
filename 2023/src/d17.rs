use crate::{graph::Digraph, io::read_lines};

aoc::test::test_part!(test1, part1, 970);
aoc::test::test_part!(test2, part2, 1149);

const INPUT: &str = include_str!("input/17.txt");

pub fn part1() -> usize {
	solve(1, 3)
}

pub fn part2() -> usize {
	solve(4, 10)
}

type Coords = (usize, usize, usize);

fn solve(min: usize, max: usize) -> usize {
	let costs = read_costs();
	let graph = get_graph(&costs, min, max);
	let pred = |c: &Coords| -> bool {
		c.0 == costs.len() - 1 && c.1 == costs[0].len() - 1
	};
	graph
		.shortest_distance((0, 0, 0), pred)
		.min(graph.shortest_distance((0, 0, 1), pred))
		.unwrap()
}

fn read_costs() -> Vec<Vec<usize>> {
	INPUT
		.lines()
		.map(|line| line.bytes().map(|b| (b - b'0') as usize).collect())
		.collect()
}

fn get_graph(costs: &[Vec<usize>], min: usize, max: usize) -> Digraph<Coords> {
	let (n, m) = (costs.len(), costs[0].len());
	// Model the graph as two planes. Each move must move between min and max
	// spaces vertically or horizontally (depending on the current plane) and
	// move to the other plane, to force alternating horizontal and vertical
	// move sequences.
	let mut graph = Digraph::new();
	for i in 0..n {
		for j in 0..m {
			let n0 = (i, j, 0);
			let mut cost = 0;
			for ii in (i.saturating_sub(max)..i).rev() {
				cost += costs[ii][j];
				if i.abs_diff(ii) >= min {
					graph.insert_edge(n0, (ii, j, 1), cost);
				}
			}
			let mut cost = 0;
			for (ii, c) in costs.iter().enumerate().skip(i + 1).take(max) {
				cost += c[j];
				if i.abs_diff(ii) >= min {
					graph.insert_edge(n0, (ii, j, 1), cost);
				}
			}
			let n1 = (i, j, 1);
			let mut cost = 0;
			for jj in (j.saturating_sub(max)..j).rev() {
				cost += costs[i][jj];
				if j.abs_diff(jj) >= min {
					graph.insert_edge(n1, (i, jj, 0), cost);
				}
			}
			let mut cost = 0;
			for jj in j + 1..=(j + max).min(m - 1) {
				cost += costs[i][jj];
				if j.abs_diff(jj) >= min {
					graph.insert_edge(n1, (i, jj, 0), cost);
				}
			}
		}
	}
	graph
}

#[derive(PartialEq, Eq)]
struct State {
	cost: usize,
	coords: Coords,
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		other
			.cost
			.cmp(&self.cost)
			.then_with(|| self.coords.cmp(&other.coords))
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}
