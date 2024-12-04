use aoc_2024::day_1::{parser, sorter};

fn main() -> anyhow::Result<()> {
	let contents = aoc_2024::read_from_args("day_1_part_1", "locations")?;

	let locations = parser::parse_pairs_tuple(&contents)?;
	let locations = sorter::sort_hashmap(locations);

	let total_difference = locations
		.iter()
		.map(|(&a, &b)| {
			a.abs_diff(b)
		})
		.sum::<u32>();

	println!("The total difference is: {total_difference}");

	Ok(())
}
