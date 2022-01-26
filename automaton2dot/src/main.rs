use fsm::Automaton;
use std::io;

fn main() {
	if let Err(err) = (|| {
		Automaton::load_from_simple_text(&mut io::stdin().lock())?.store_as_dot(&mut io::stdout())
	})() {
		eprintln!("Error: {}", err);
		std::process::exit(1);
	}
}
