use super::{matrix, DFA};
use ndarray::ArrayView;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, VecDeque};

impl DFA {
	pub fn reachable_from(&self, start_state: usize) -> Self {
		let mut state_map = vec![usize::MAX; self.n_states];
		let mut reachable = 0;
		let mut queue = VecDeque::new();
		if start_state < self.n_states {
			state_map[start_state] = reachable;
			reachable += 1;
			queue.push_back(start_state);
		}
		let mut output_matrix = matrix::empty_output(self.model, self.n_inputs);
		let mut state_matrix = matrix::empty_state(self.n_inputs);
		while let Some(old_state) = queue.pop_front() {
			let old_state_row = self.state_matrix.row(old_state);
			for &old_state in old_state_row {
				if state_map[old_state] == usize::MAX {
					state_map[old_state] = reachable;
					reachable += 1;
					queue.push_back(old_state);
				}
			}
			output_matrix
				.push_row(self.output_matrix.row(old_state))
				.unwrap();
			state_matrix.push_row(old_state_row).unwrap();
			state_matrix
				.row_mut(state_map[old_state])
				.mapv_inplace(|old_state| state_map[old_state]);
		}
		Self {
			model: self.model,
			n_states: reachable,
			n_inputs: self.n_inputs,
			n_outputs: self.n_outputs,
			output_matrix,
			state_matrix,
		}
	}

	fn initial_partitioning(&self) -> (Vec<usize>, usize) {
		let mut partitioning = Vec::with_capacity(self.n_states);
		let mut class_map = BTreeMap::new();
		for output_row in self.output_matrix.rows() {
			let next_class = class_map.len();
			partitioning.push(
				*class_map
					.entry(output_row.to_slice().unwrap())
					.or_insert(next_class),
			);
		}
		(partitioning, class_map.len())
	}

	pub fn minimize(&self) -> Self {
		let (mut partitioning, classes_count) = self.initial_partitioning();
		let mut class_occurrence = vec![false; classes_count];
		loop {
			let mut is_modified = false;
			let mut class_map = BTreeMap::new();
			let class_matrix = self.state_matrix.mapv(|state| partitioning[state]);
			for (state, class) in partitioning.iter_mut().enumerate() {
				match class_map.entry((*class, class_matrix.row(state).to_slice().unwrap())) {
					Entry::Vacant(entry) => {
						if !class_occurrence[*class] {
							class_occurrence[*class] = true;
						} else {
							is_modified = true;
							*class = class_occurrence.len();
							class_occurrence.push(true);
						}
						entry.insert((*class, state));
					}
					Entry::Occupied(entry) => {
						let new_class = entry.get().0;
						if new_class != *class {
							*class = new_class;
						}
					}
				}
			}
			if !is_modified {
				let mut output_matrix = matrix::empty_output(self.model, self.n_inputs);
				let mut state_matrix = matrix::empty_state(self.n_inputs);
				for ((_, class_row), (_, state)) in class_map {
					state_matrix
						.push_row(ArrayView::from_shape(self.n_inputs, class_row).unwrap())
						.unwrap();
					output_matrix
						.push_row(self.output_matrix.row(state))
						.unwrap();
				}
				return Self {
					model: self.model,
					n_states: class_occurrence.len(),
					n_inputs: self.n_inputs,
					n_outputs: self.n_outputs,
					output_matrix,
					state_matrix,
				};
			}
			class_occurrence.fill(false);
		}
	}
}
