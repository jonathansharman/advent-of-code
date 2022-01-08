/// The indices of the four-direction neighbors of element (`i`, `j`) in a grid
/// of size (`row_count`, `col_count`).
pub fn four(row_count: usize, col_count: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
	let mut result = Vec::new();
	if 0 < i && i < row_count {
		result.push((i - 1, j));
	}
	if i < row_count - 1 {
		result.push((i + 1, j));
	}
	if 0 < j && j < col_count {
		result.push((i, j - 1));
	}
	if j < col_count - 1 {
		result.push((i, j + 1));
	}
	result
}

/// The indices of the eight-direction neighbors of element (`i`, `j`) in a grid
/// of size (`row_count`, `col_count`).
pub fn eight(row_count: usize, col_count: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
	let mut result = Vec::new();
	if 0 < i && i < row_count {
		result.push((i - 1, j));
		if 0 < j && j < col_count {
			result.push((i - 1, j - 1));
		}
		if j < col_count - 1 {
			result.push((i - 1, j + 1));
		}
	}
	if i < row_count - 1 {
		result.push((i + 1, j));
		if 0 < j && j < col_count {
			result.push((i + 1, j - 1));
		}
		if j < col_count - 1 {
			result.push((i + 1, j + 1));
		}
	}
	if 0 < j && j < col_count {
		result.push((i, j - 1));
	}
	if j < col_count - 1 {
		result.push((i, j + 1));
	}
	result
}
