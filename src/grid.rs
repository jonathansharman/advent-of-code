use std::ops::{Index, IndexMut};

use crate::define_point_and_vector;

define_point_and_vector!(Point, Vector, row, col, i64);

/// A rectangular grid of tiles.
#[derive(Clone, Debug)]
pub struct Grid<T> {
	tiles: Vec<Vec<T>>,
}

impl<T> Grid<T> {
	/// Creates a new grid with the given `size` and `default` value.
	pub fn new(dimensions: Vector, default: T) -> Grid<T>
	where
		T: Clone,
	{
		Grid {
			tiles: vec![
				vec![default.clone(); dimensions.col as usize];
				dimensions.row as usize
			],
		}
	}

	// An iterator over the grid's coordinate-tile pairs.
	pub fn tiles(&self) -> impl Iterator<Item = (Point, &T)> {
		self.rows().enumerate().flat_map(|(i, row)| {
			row.iter()
				.enumerate()
				.map(move |(j, tile)| (Point::from((i, j)), tile))
		})
	}

	// An iterator over mutable references to the grid's coordinate-tile pairs.
	pub fn tiles_mut(&mut self) -> impl Iterator<Item = (Point, &mut T)> {
		self.rows_mut().enumerate().flat_map(|(i, row)| {
			row.iter_mut()
				.enumerate()
				.map(move |(j, tile)| (Point::from((i, j)), tile))
		})
	}

	// Converts the grid into an iterator over its coordinate-tile pairs.
	pub fn into_tiles(self) -> impl Iterator<Item = (Point, T)> {
		self.into_rows().enumerate().flat_map(|(i, row)| {
			row.into_iter()
				.enumerate()
				.map(move |(j, tile)| (Point::from((i, j)), tile))
		})
	}

	// An iterator over the coordinate-tile pairs orthogonally adjacent to
	// `coords`.
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

	// An iterator over the coordinate-tile pairs orthogonally or diagonally
	// adjacent to `coords`.
	pub fn eight_neighbors(
		&self,
		coords: Point,
	) -> impl Iterator<Item = (Point, &T)> {
		(-1..=1).flat_map(move |i| {
			(-1..=1).filter_map(move |j| {
				if i == 0 && j == 0 {
					return None;
				}
				let neighbor = coords + (i, j).into();
				self.get(neighbor).map(|value| (neighbor, value))
			})
		})
	}

	/// An iterator over the rows of the grid.
	pub fn rows(&self) -> impl Iterator<Item = &Vec<T>> {
		self.tiles.iter()
	}

	/// An iterator over mutable references to the rows of the grid.
	pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<T>> {
		self.tiles.iter_mut()
	}

	/// Converts the grid into an iterator over its rows.
	pub fn into_rows(self) -> impl Iterator<Item = Vec<T>> {
		self.tiles.into_iter()
	}

	/// An iterator over the elements of the row at `row_idx`, if there is one.
	pub fn get_row(&self, row_idx: i64) -> Option<impl Iterator<Item = &T>> {
		(0..=self.height())
			.contains(&row_idx)
			.then(|| self.tiles[row_idx as usize].iter())
	}

	/// An iterator over the columns of the grid. Note that because the grid is
	/// stored in row-major order, this is less efficient than iterating over
	/// the rows: it performs one allocation per column.
	pub fn cols(&self) -> impl Iterator<Item = Vec<&T>> {
		(0..self.width()).map(|col_idx| {
			(0..self.height())
				.map(|row_idx| &self[(row_idx, col_idx).into()])
				.collect()
		})
	}

	/// An iterator over the elements of the column at `col_idx`, if there is
	/// one.
	pub fn get_col(&self, col_idx: i64) -> Option<impl Iterator<Item = &T>> {
		(0..=self.width()).contains(&col_idx).then(move || {
			(0..self.height()).map(move |row_idx| {
				&self.tiles[row_idx as usize][col_idx as usize]
			})
		})
	}

	/// A reference to the element at `coords`, if in bounds.
	pub fn get(&self, coords: Point) -> Option<&T> {
		let row: usize = coords.row.try_into().ok()?;
		let col: usize = coords.col.try_into().ok()?;
		self.tiles.get(row).and_then(|row| row.get(col))
	}

	/// A mutable reference to the element at `coords`, if in bounds.
	pub fn get_mut(&mut self, coords: Point) -> Option<&mut T> {
		let row: usize = coords.row.try_into().ok()?;
		let col: usize = coords.col.try_into().ok()?;
		self.tiles.get_mut(row).and_then(|row| row.get_mut(col))
	}

	/// Whether `coords` is in bounds.
	pub fn contains(&self, coords: Point) -> bool {
		self.get(coords).is_some()
	}

	/// The number of rows in the grid.
	pub fn height(&self) -> i64 {
		self.tiles.len() as i64
	}

	/// The number of columns in the grid. The grid must have at least one row.
	/// This assumes that all rows have the same width.
	pub fn width(&self) -> i64 {
		self.tiles[0].len() as i64
	}

	/// The grid's row-column dimensions. The grid must have at least one row.
	/// This assumes that all rows have the same width.
	pub fn dimensions(&self) -> Vector {
		Vector {
			row: self.height(),
			col: self.width(),
		}
	}

	/// The grid's transpose, i.e. the reflection of the grid's tiles over its
	/// diagonal.
	pub fn transpose(&self) -> Grid<T>
	where
		T: Clone,
	{
		Grid {
			tiles: self
				.cols()
				.map(|col| col.into_iter().cloned().collect())
				.collect(),
		}
	}

	/// Reverses the order of the grid's rows in-place.
	pub fn flip_vertically(&mut self) {
		self.tiles.reverse();
	}

	/// Reverses the order of the grid's columns in-place.
	pub fn flip_horizontally(&mut self) {
		for row in &mut self.tiles {
			row.reverse();
		}
	}

	/// Rotates the grid's tiles clockwise.
	pub fn rotate_cw(&mut self)
	where
		T: Clone,
	{
		self.flip_vertically();
		*self = self.transpose();
	}

	/// Rotates the grid's tiles counterclockwise.
	pub fn rotate_ccw(&mut self)
	where
		T: Clone,
	{
		*self = self.transpose();
		self.flip_vertically();
	}
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
	fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
		Self {
			tiles: iter
				.into_iter()
				.map(|row| row.into_iter().collect())
				.collect(),
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
