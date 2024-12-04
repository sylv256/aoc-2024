/// Whether the intervals between each of these levels are safe
/// and they're either all increasing or all decreasing.
pub fn is_safe(levels: &Vec<i32>) -> bool {
	let all_increasing = levels
		.windows(2)
		.all(|x| x[1] - x[0] > 0);
	let all_decreasing = levels
		.windows(2)
		.all(|x| x[1] - x[0] < 0);
	let safe_intervals = levels
		.windows(2)
		.all(|x| {
			let dist = x[0].abs_diff(x[1]);
			dist <= 3 && dist >= 1
		});

	safe_intervals && (all_increasing || all_decreasing)
}
