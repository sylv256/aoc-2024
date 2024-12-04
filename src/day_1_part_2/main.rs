use std::collections::HashMap;

use aoc_2024::day_1::parser;

fn main() -> anyhow::Result<()> {
	let contents = aoc_2024::read_from_args("day_1_part_2", "locations")?;

	let locations = parser::parse_pairs(&contents)?;

	// Get left side
	let left = locations
		.chunks(2)
		.map(|x| x[0])
		.collect::<Vec<i32>>();

	// Get right side
	let right = locations
		.chunks(2)
		.map(|x| x[1])
		.collect::<Vec<i32>>();

	// Deduplicate left side in order to iterate over each kind of number
	let mut dedup_left = left.clone();
	dedup_left.dedup();

	// Count each number on the right side
	let mut counts = HashMap::<i32, i32>::new();
	dedup_left
		.iter()
		.for_each(|b| {
			let count = right
				.iter()
				.filter(|a| **a == *b)
				.count();
			counts.insert(*b, count as i32);
		});

	// Calculate the similarity score
	let similarity_score = left
		.iter()
		.filter_map(|x| Some(x * counts.get(x)?))
		.sum::<i32>();

	println!("The similarity score is: {similarity_score}");

	Ok(())
}
