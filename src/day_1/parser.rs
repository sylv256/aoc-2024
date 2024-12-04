use crate::{ParseError, ParseResult};

/// Parse contents and pair them up
pub fn parse_pairs_tuple(text: &str) -> ParseResult<Vec<(i32, i32)>> {
	let pairs = parse_pairs(text)?
		.chunks(2)
		.map(|a| (a[0], a[1]))
		.collect::<Vec<(i32, i32)>>();
	Ok(pairs)
}

/// Parse contents
pub fn parse_pairs(text: &str) -> ParseResult<Vec<i32>> {
	let pairs = text
		.split_whitespace()
		.filter(|x| !x.is_empty())
		.map(|x| x.parse::<i32>())
		.collect::<Result<Vec<i32>, _>>()?;
	if pairs.len() % 2 != 0 {
		return Err(ParseError::OddArguments(pairs.len()))
	}

	Ok(pairs)
}
