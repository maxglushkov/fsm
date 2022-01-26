use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Model {
	Mealy,
	Moore,
}

pub struct ParseModelError {}

impl FromStr for Model {
	type Err = ParseModelError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_str() {
			"ml" | "mealy" => Ok(Model::Mealy),
			"mr" | "moore" => Ok(Model::Moore),
			_ => Err(ParseModelError {}),
		}
	}
}

impl fmt::Display for Model {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(match self {
			Model::Mealy => "Mealy",
			Model::Moore => "Moore",
		})
	}
}

impl fmt::Display for ParseModelError {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("unsupported state machine model")
	}
}
