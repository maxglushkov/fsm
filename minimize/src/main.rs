use fsm::DFA;
use std::io;

fn main() {
	if let Err(err) = (|| {
		DFA::load_from_simple_text(&mut io::stdin().lock())?
			.reachable_from(0)
			.minimize()
			.store_as_simple_text(&mut io::stdout())
	})() {
		eprintln!("Error: {}", err);
		std::process::exit(1);
	}
}
