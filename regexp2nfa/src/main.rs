use fsm::Automaton;
use std::io;

fn main() {
	if let Err(err) = (|| {
		let mut line = String::new();
		io::stdin().read_line(&mut line)?;
		Automaton::from_regexp(line.trim_end())
			.map_err(|err| {
				io::Error::new(io::ErrorKind::InvalidData, format!("{{stdin}}:1:{}", err))
			})?
			.store_as_simple_text(&mut io::stdout())
	})() {
		eprintln!("Error: {}", err);
		std::process::exit(1);
	}
}
