use std::fmt::Display;
use std::io::{BufRead, Error, ErrorKind, Result};
use std::str::FromStr;

pub fn parse_array0<R: BufRead, T: FromStr>(reader: &mut R, line: &mut String) -> Result<T>
where
	T::Err: Display,
{
	reader.read_line(line)?;
	let parsed = line.trim().parse();
	line.clear();
	parsed.map_err(|err: T::Err| Error::new(ErrorKind::InvalidData, err.to_string()))
}
