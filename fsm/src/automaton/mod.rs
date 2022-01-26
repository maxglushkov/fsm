mod determine;
mod dot;
mod regexp;
mod regular_grammar;
mod simple_text;
mod state;
use state::StateType;
use std::collections::BTreeSet;

pub struct Automaton {
	states: Vec<String>,
	inputs: Vec<char>,
	transitions: BTreeSet<(usize, usize, usize)>,
}
