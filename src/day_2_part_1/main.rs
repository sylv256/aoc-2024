use aoc_2024::day_2::{parser, safety};

fn main() -> anyhow::Result<()> {
	let contents = aoc_2024::read_from_args("day_2_part_1", "levels_table")?;

	let levels_table = parser::parse(&contents)?;

	let safe_levels = levels_table
		.iter()
		.filter(|levels| safety::is_safe(levels))
		.count();

	println!("The amount of safe levels is/are {safe_levels}");

	Ok(())
}
