use std::collections::{BinaryHeap, HashMap};

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 970);
crate::test::test_part!(test2, part2, 1149);

pub fn part1() -> usize {
	let costs = read_costs();
	let adj = get_adjacencies(&costs, 1, 3);
	shortest_path(&costs, &adj, 0).min(shortest_path(&costs, &adj, 1))
}

pub fn part2() -> usize {
	let costs = read_costs();
	let adj = get_adjacencies(&costs, 4, 10);
	shortest_path(&costs, &adj, 0).min(shortest_path(&costs, &adj, 1))
}

fn read_costs() -> Vec<Vec<usize>> {
	read_lines("input/2023/17.txt")
		.map(|line| line.bytes().map(|b| (b - b'0') as usize).collect())
		.collect()
}

type Coords = (usize, usize, usize);

fn get_adjacencies(
	costs: &[Vec<usize>],
	min: usize,
	max: usize,
) -> HashMap<Coords, Vec<(Coords, usize)>> {
	let (n, m) = (costs.len(), costs[0].len());
	// Model the graph as two planes. Each move must move between min and max
	// spaces vertically or horizontally (depending on the current plane) and
	// move to the other plane, to force alternating horizontal and vertical
	// move sequences.
	let mut adj: HashMap<Coords, Vec<(Coords, usize)>> = HashMap::new();
	for i in 0..n {
		for j in 0..m {
			let adj0 = adj.entry((i, j, 0)).or_default();
			let mut cost = 0;
			for ii in (i.saturating_sub(max)..i).rev() {
				cost += costs[ii][j];
				if i.abs_diff(ii) >= min {
					adj0.push(((ii, j, 1), cost));
				}
			}
			let mut cost = 0;
			for (ii, c) in costs.iter().enumerate().skip(i + 1).take(max) {
				cost += c[j];
				if i.abs_diff(ii) >= min {
					adj0.push(((ii, j, 1), cost));
				}
			}
			let adj1 = adj.entry((i, j, 1)).or_default();
			let mut cost = 0;
			for jj in (j.saturating_sub(max)..j).rev() {
				cost += costs[i][jj];
				if j.abs_diff(jj) >= min {
					adj1.push(((i, jj, 0), cost));
				}
			}
			let mut cost = 0;
			for jj in j + 1..=(j + max).min(m - 1) {
				cost += costs[i][jj];
				if j.abs_diff(jj) >= min {
					adj1.push(((i, jj, 0), cost));
				}
			}
		}
	}
	adj
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
