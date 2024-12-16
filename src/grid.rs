use std::{
	fmt::{Debug, Write},
	ops::{Index, IndexMut},
};

use crate::define_point_and_vector;

define_point_and_vector!(Point, Vector, row, col, i64);

/// A rectangular grid of tiles.
#[derive(Clone)]
pub struct Grid<T> {
	tiles: Vec<T>,
	size: Vector,
}

impl<T> Grid<T> {
	/// Creates a new grid with the given `size` and `default` value.
	pub fn new(size: Vector, default: T) -> Grid<T>
	where
		T: Clone,
	{
		Grid {
			tiles: vec![default.clone(); (size.row * size.col) as usize],
			size,
		}
	}

	/// An iterator over the grid's coordinate-tile pairs.
	pub fn iter(&self) -> GridIter<'_, T> {
		self.into_iter()
	}

	/// Converts the grid into a vector of all of its tiles.
	pub fn into_tiles(self) -> Vec<T> {
		self.tiles
	}

	// TODO: Implement Tiles and TilesMut iterators, similar to Rows and
	// RowsMut, and use them to implement tiles and tiles_mut?

	/// A slice of all the tiles in the grid.
	pub fn tiles(&self) -> &[T] {
		&self.tiles
	}

	/// Mutable references to all the tiles in the grid.
	pub fn tiles_mut(&mut self) -> &mut [T] {
		&mut self.tiles
	}

	/// An iterator over the coordinate-tile pairs orthogonally adjacent to
	/// `coords`.
	pub fn four_neighbors(
		&self,
		coords: Point,
	) -> impl Iterator<Item = (Point, &T)> {
		[(-1, 0), (0, -1), (0, 1), (1, 0)].into_iter().filter_map(
			move |offset| {
				let neighbor = coords + offset.into();
				self.get(neighbor).map(|value| (neighbor, value))
			},
		)
	}

	/// An iterator over the coordinate-tile pairs orthogonally or diagonally
	/// adjacent to `coords`.
	pub fn eight_neighbors(
		&self,
		coords: Point,
	) -> impl Iterator<Item = (Point, &T)> {
		(-1..=1).flat_map(move |i| {
			(-1..=1).filter_map(move |j| {
				if i == 0 && j == 0 {
					return None;
				}
				let neighbor = coords + Vector::new(i, j);
				self.get(neighbor).map(|value| (neighbor, value))
			})
		})
	}

	/// An iterator over the rows of the grid.
	pub fn rows(&self) -> impl Iterator<Item = Row<'_, T>> {
		self.tiles
			.chunks_exact(self.size.col as usize)
			.map(|chunk| Row { tiles: chunk })
	}

	/// An iterator over mutable references to the rows of the grid.
	pub fn rows_mut(&mut self) -> impl Iterator<Item = RowMut<'_, T>> {
		self.tiles
			.chunks_exact_mut(self.size.col as usize)
			.map(|chunk| RowMut { tiles: chunk })
	}

	/// The elements of the row at `row_idx`, if there is one.
	pub fn get_row(&self, row_idx: i64) -> Option<Row<'_, T>> {
		(0..self.row_count()).contains(&row_idx).then(|| {
			let start = (row_idx * self.size.col) as usize;
			let end = ((row_idx + 1) + self.size.col) as usize;
			Row {
				tiles: &self.tiles[start..end],
			}
		})
	}

	/// Mutable references to the elements of the row at `row_idx`, if there is
	/// one.
	pub fn get_row_mut(&mut self, row_idx: i64) -> Option<RowMut<'_, T>> {
		(0..self.row_count()).contains(&row_idx).then(|| {
			let start = (row_idx * self.size.col) as usize;
			let end = ((row_idx + 1) + self.size.col) as usize;
			RowMut {
				tiles: &mut self.tiles[start..end],
			}
		})
	}

	/// An iterator over the columns of the grid.
	pub fn cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
		(0..self.size.col).map(|col_idx| {
			self.tiles
				.iter()
				.skip(col_idx as usize)
				.step_by(self.size.col as usize)
		})
	}

	// TODO: Implement Cols, ColsMut, Col, and ColMut, and use ColsMut to
	// implement a cols_mut. Since columns don't have contiguous storage, ColMut
	// and especially ColsMut may require unsafe code.

	/// An iterator over the elements of the column at `col_idx`, if there is
	/// one.
	pub fn get_col(&self, col_idx: i64) -> Option<impl Iterator<Item = &T>> {
		(0..=self.size.col).contains(&col_idx).then(move || {
			self.tiles
				.iter()
				.skip(col_idx as usize)
				.step_by(self.size.col as usize)
		})
	}

	/// An iterator over mutable references to the elements of the column at
	/// `col_idx`, if there is one.
	pub fn get_col_mut(
		&mut self,
		col_idx: i64,
	) -> Option<impl Iterator<Item = &mut T>> {
		(0..=self.size.col).contains(&col_idx).then(move || {
			self.tiles
				.iter_mut()
				.skip(col_idx as usize)
				.step_by(self.size.col as usize)
		})
	}

	/// A reference to the element at `coords`, if in bounds.
	pub fn get(&self, coords: Point) -> Option<&T> {
		self.tile_index(coords).map(|idx| &self.tiles[idx])
	}

	/// A mutable reference to the element at `coords`, if in bounds.
	pub fn get_mut(&mut self, coords: Point) -> Option<&mut T> {
		self.tile_index(coords).map(|idx| &mut self.tiles[idx])
	}

	/// Swaps the tiles at `coords1` and `coords2`, if they're both in-bounds.
	pub fn swap(&mut self, coords1: Point, coords2: Point) {
		if let (Some(idx1), Some(idx2)) =
			(self.tile_index(coords1), self.tile_index(coords2))
		{
			self.tiles.swap(idx1, idx2);
		}
	}

	/// Whether `coords` is in bounds.
	pub fn contains_coords(&self, coords: Point) -> bool {
		self.get(coords).is_some()
	}

	/// The number of rows in the grid.
	pub fn row_count(&self) -> i64 {
		self.size.row
	}

	/// The number of columns in the grid.
	pub fn col_count(&self) -> i64 {
		self.size.col
	}

	/// The grid's row-column dimensions.
	pub fn size(&self) -> Vector {
		self.size
	}

	/// Transposes the grid in-place, reflecting its tiles over its diagonal.
	pub fn transpose(&mut self)
	where
		T: Clone,
	{
		// TODO: Swap elements in-place, and remove the Clone bound on T.
		*self = Grid {
			tiles: self.cols().fold(Vec::new(), |mut acc, col| {
				acc.extend(col.cloned());
				acc
			}),
			size: Vector {
				row: self.size.col,
				col: self.size.row,
			},
		};
	}

	/// Reverses the order of the grid's rows in-place.
	pub fn flip_vertically(&mut self) {
		for row_idx in 0..self.size.row / 2 {
			let other_row_idx = self.size.row - 1 - row_idx;
			for col_idx in 0..self.size.col {
				self.swap(
					Point::new(row_idx, col_idx),
					Point::new(other_row_idx, col_idx),
				);
			}
		}
	}

	/// Reverses the order of the grid's columns in-place.
	pub fn flip_horizontally(&mut self) {
		for row in self.rows_mut() {
			row.tiles.reverse();
		}
	}

	/// Rotates the grid's tiles clockwise.
	pub fn rotate_cw(&mut self)
	where
		T: Clone,
	{
		self.flip_vertically();
		self.transpose();
	}

	/// Rotates the grid's tiles counterclockwise.
	pub fn rotate_ccw(&mut self)
	where
		T: Clone,
	{
		self.transpose();
		self.flip_vertically();
	}

	/// The index of `coords` within the underlying tile vector.
	fn tile_index(&self, coords: Point) -> Option<usize> {
		((0..self.size.row).contains(&coords.row)
			&& (0..self.size.col).contains(&coords.col))
		.then_some((coords.row * self.size.col + coords.col) as usize)
	}
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
	fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
		let tiles: Vec<Vec<T>> = iter
			.into_iter()
			.map(|row| row.into_iter().collect())
			.collect();
		let size = Vector {
			row: tiles.len() as i64,
			col: tiles[0].len() as i64,
		};
		Self {
			tiles: tiles.into_iter().fold(Vec::new(), |mut acc, row| {
				acc.extend(row);
				acc
			}),
			size,
		}
	}
}

/// An iterator over a [`Grid`]'s coordinate-tile pairs.
pub struct GridIter<'a, T> {
	grid: &'a Grid<T>,
	coords: Point,
}

impl<'a, T> Iterator for GridIter<'a, T> {
	type Item = (Point, &'a T);

	fn next(&mut self) -> Option<Self::Item> {
		let tile = self.grid.get(self.coords)?;
		let coords = self.coords;

		if self.coords.col < self.grid.size.col {
			self.coords.col += 1;
		}
		if self.coords.col == self.grid.size.col {
			self.coords.col = 0;
			self.coords.row += 1;
		}

		Some((coords, tile))
	}
}

impl<'a, T> IntoIterator for &'a Grid<T> {
	type Item = (Point, &'a T);

	type IntoIter = GridIter<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		GridIter {
			grid: self,
			coords: Point::zero(),
		}
	}
}

// TODO: Implement Grid::iter_mut and IntoIterator for &mut Grid using
// Grid::iter_mut once the use of impl trait in associated types is stabilized.
// At the same time, could probably get rid of GridIter and reimplement Iterator
// for &Grid more simply using Grid::iter.

impl<'a, T> IntoIterator for &'a mut Grid<T> {
	type Item = (Point, &'a T);

	type IntoIter = GridIter<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		GridIter {
			grid: self,
			coords: Point::zero(),
		}
	}
}

impl<T> Index<Point> for Grid<T> {
	type Output = T;

	fn index(&self, index: Point) -> &T {
		self.get(index).unwrap()
	}
}

impl<T> IndexMut<Point> for Grid<T> {
	fn index_mut(&mut self, index: Point) -> &mut T {
		self.get_mut(index).unwrap()
	}
}

impl<T: Debug> Debug for Grid<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in self.rows() {
			for tile in row {
				f.write_fmt(format_args!("{:?}", tile))?;
			}
			f.write_char('\n')?;
		}
		Ok(())
	}
}

/// A reference to a [`Grid`] row.
pub struct Row<'a, T> {
	tiles: &'a [T],
}

impl<T> Row<'_, T> {
	/// Whether the row has no tiles.
	pub fn is_empty(&self) -> bool {
		self.tiles.is_empty()
	}

	/// The number of tiles in the row.
	pub fn len(&self) -> usize {
		self.tiles.len()
	}
}

impl<'a, T> Iterator for Row<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		let (first, rest) = self.tiles.split_first()?;
		self.tiles = rest;
		Some(first)
	}
}

impl<T, Idx> Index<Idx> for Row<'_, T>
where
	Idx: std::slice::SliceIndex<[T]>,
{
	type Output = Idx::Output;

	fn index(&self, index: Idx) -> &Self::Output {
		&self.tiles[index]
	}
}

/// A mutable reference to a [`Grid`] row.
pub struct RowMut<'a, T> {
	tiles: &'a mut [T],
}

impl<T> RowMut<'_, T> {
	/// Whether the row has no tiles.
	pub fn is_empty(&self) -> bool {
		self.tiles.is_empty()
	}

	/// The number of tiles in the row.
	pub fn len(&self) -> usize {
		self.tiles.len()
	}
}

impl<'a, T> Iterator for RowMut<'a, T> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		let tiles = std::mem::take(&mut self.tiles);
		let (first, rest) = tiles.split_first_mut()?;
		self.tiles = rest;
		Some(first)
	}
}

impl<T, Idx> Index<Idx> for RowMut<'_, T>
where
	Idx: std::slice::SliceIndex<[T]>,
{
	type Output = Idx::Output;

	fn index(&self, index: Idx) -> &Self::Output {
		&self.tiles[index]
	}
}

impl<T, Idx> IndexMut<Idx> for RowMut<'_, T>
where
	Idx: std::slice::SliceIndex<[T]>,
{
	fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
		&mut self.tiles[index]
	}
}
