use crate::ParseResult;

pub fn parse(text: &str) -> ParseResult<Vec<Vec<i32>>> {
	let lines = text
		.lines()
		.filter(|x| !x.is_empty())
		.map(|x| {
			x
				.split_whitespace()
				.map(|x| x.parse::<i32>())
				.collect::<Result<Vec<i32>, _>>()
		})
		.collect::<Result<Vec<Vec<i32>>, _>>()?;
	Ok(lines)
}
