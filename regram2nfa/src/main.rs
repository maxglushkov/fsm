use fsm::Automaton;
use std::io;

fn main() {
	if let Err(err) = (|| {
		Automaton::load_from_regular_grammar(&mut io::stdin().lock())?
			.store_as_simple_text(&mut io::stdout())
	})() {
		eprintln!("Error: {}", err);
		std::process::exit(1);
	}
}
