#![allow(clippy::tabs_in_doc_comments)] // https://adamtuttle.codes/blog/2021/tabs-vs-spaces-its-an-accessibility-issue/

pub mod day_1;
pub mod day_2;

use std::{env, fs, process};
use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
	#[error("error parsing int: {0}")]
	ParseIntError(#[from] ParseIntError),
	#[error("odd number of arguments in key-value pair list: {0}")]
	OddArguments(usize),
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("wrong number of arguments")]
	ArgsError,
}

pub fn read_from_args(app_name: &'static str, input_name: &'static str) -> anyhow::Result<String> {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("Usage:\n\
{app_name} <{input_name}>");
		drop(args);
		process::abort()
	}
	let path = &args[1];
	Ok(fs::read_to_string(path)?)
}
