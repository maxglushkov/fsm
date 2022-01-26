use super::{Automaton, StateType};
use std::collections::hash_map::Entry;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::rc::Rc;

fn get_or_create_state(
	states: &mut HashMap<Rc<Vec<usize>>, usize>,
	queue: &mut VecDeque<Rc<Vec<usize>>>,
	old_states: BTreeSet<usize>,
) -> usize {
	let next_id = states.len();
	let old_states = Rc::new(old_states.into_iter().collect());
	match states.entry(old_states) {
		Entry::Vacant(entry) => {
			queue.push_back(entry.key().clone());
			entry.insert(next_id);
			next_id
		}
		Entry::Occupied(entry) => *entry.get(),
	}
}

fn format_state_name(new_state: usize, old_states: &[usize], old_names: &[String]) -> String {
	let mut state_type = StateType::default();
	for &old_state in old_states {
		state_type |= StateType::from_last_char(&old_names[old_state]);
	}
	if new_state != 0 {
		state_type &= StateType::FINAL;
	}
	format!("S{}{}", new_state, state_type.name())
}

impl Automaton {
	fn initial_states(&self) -> BTreeSet<usize> {
		let mut states = BTreeSet::new();
		for (id, name) in self.states.iter().enumerate() {
			if StateType::from_last_char(name) & StateType::INITIAL {
				states.insert(id);
			}
		}
		states
	}

	pub fn determine(&self) -> Self {
		let mut automaton = Self {
			states: Vec::new(),
			inputs: self.inputs.clone(),
			transitions: BTreeSet::new(),
		};
		let mut states = HashMap::new();
		let mut queue = VecDeque::new();
		get_or_create_state(&mut states, &mut queue, self.initial_states());
		while let Some(old_states) = queue.pop_front() {
			let from = automaton.states.len();
			automaton
				.states
				.push(format_state_name(from, &old_states, &self.states));
			let mut old_transitions = BTreeMap::new();
			for &from in old_states.iter() {
				for (_, on, into) in self.transitions.range((from, 0, 0)..(from + 1, 0, 0)) {
					old_transitions
						.entry(on)
						.or_insert_with(BTreeSet::new)
						.insert(*into);
				}
			}
			for (&on, into) in old_transitions {
				let into = get_or_create_state(&mut states, &mut queue, into);
				automaton.transitions.insert((from, on, into));
			}
		}
		automaton
	}
}
