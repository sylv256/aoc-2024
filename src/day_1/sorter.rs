use std::collections::HashMap;

/// Sorts locations and puts them in a HashMap
pub fn sort_hashmap(locations: Vec<(i32, i32)>) -> HashMap<i32, i32> {
	sort(locations)
		.into_iter()
		.collect()
}

/// Sorts locations
pub fn sort(mut locations: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
	// Sort left-side locations
	locations.sort_by(|(a, _), (c, _)| a.cmp(c));

	// Sort right-side locations
	let mut sorted_right = locations
		.iter()
		.map(|(_, b)| *b)
		.collect::<Vec<i32>>();
	sorted_right.sort();

	// Join `sorted_right` with `locations`
	locations
		.into_iter()
		.enumerate()
		.map(|(i, (a, _))| (a, sorted_right[i]))
		.collect::<Vec<(i32, i32)>>()
}
