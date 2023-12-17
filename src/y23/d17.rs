use std::collections::{BinaryHeap, HashMap};

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 970);
crate::test::test_part!(test2, part2, ?);

type Coords = (usize, usize, usize);

pub fn part1() -> usize {
	let costs: Vec<Vec<usize>> = read_lines("input/2023/17.txt")
		.map(|line| line.bytes().map(|b| (b - b'0') as usize).collect())
		.collect();
	let (n, m) = (costs.len(), costs[0].len());
	// Model the graph as two planes. Each move must move 1-3 spaces vertically
	// or horizontally (depending on the current plane) and move to the other
	// plane, to force alternating horizontal and vertical move sequences.
	let mut adj: HashMap<Coords, Vec<(Coords, usize)>> = HashMap::new();
	for i in 0..n {
		for j in 0..m {
			let mut cost = 0;
			for ii in (i.saturating_sub(3)..i).rev() {
				cost += costs[ii][j];
				adj.entry((i, j, 0)).or_default().push(((ii, j, 1), cost));
			}
			let mut cost = 0;
			for ii in i + 1..=(i + 3).min(n - 1) {
				cost += costs[ii][j];
				adj.entry((i, j, 0)).or_default().push(((ii, j, 1), cost));
			}
			let mut cost = 0;
			for jj in (j.saturating_sub(3)..j).rev() {
				cost += costs[i][jj];
				adj.entry((i, j, 1)).or_default().push(((i, jj, 0), cost));
			}
			let mut cost = 0;
			for jj in j + 1..=(j + 3).min(m - 1) {
				cost += costs[i][jj];
				adj.entry((i, j, 1)).or_default().push(((i, jj, 0), cost));
			}
		}
	}
	shortest_path(&costs, &adj, 0).min(shortest_path(&costs, &adj, 1))
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

fn shortest_path(
	costs: &[Vec<usize>],
	adj: &HashMap<Coords, Vec<(Coords, usize)>>,
	starting_plane: usize,
) -> usize {
	let (n, m) = (costs.len(), costs[0].len());
	// (coordinates, distance from start)
	let mut distance: HashMap<Coords, usize> = HashMap::new();
	// (lowest known cost, coordinates)
	let mut queue: BinaryHeap<State> = BinaryHeap::new();
	distance.insert((0, 0, starting_plane), 0);
	queue.push(State {
		cost: 0,
		coords: (0, 0, starting_plane),
	});
	while let Some(State { cost, coords }) = queue.pop() {
		if coords.0 == n - 1 && coords.1 == m - 1 {
			return cost;
		}
		if cost > distance[&coords] {
			// We've already reached this node by a shorter path.
			continue;
		}
		for (neighbor, neighbor_cost) in adj[&coords].iter() {
			let next_cost = cost + neighbor_cost;
			let d = distance.get(neighbor);
			if d.map_or(true, |d| next_cost < *d) {
				queue.push(State {
					cost: next_cost,
					coords: *neighbor,
				});
				distance.insert(*neighbor, next_cost);
			}
		}
	}
	unreachable!();
}

pub fn part2() -> usize {
	0
}
