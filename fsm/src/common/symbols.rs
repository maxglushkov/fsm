use std::borrow::Borrow;
use std::collections::btree_map::{BTreeMap, Entry};

pub struct Symbols<T> {
	name_by_id: Vec<T>,
	id_by_name: BTreeMap<T, usize>,
}

impl<T> Symbols<T> {
	pub fn new() -> Self {
		Self {
			name_by_id: Vec::new(),
			id_by_name: BTreeMap::new(),
		}
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			name_by_id: Vec::with_capacity(capacity),
			id_by_name: BTreeMap::new(),
		}
	}

	pub fn len(&self) -> usize {
		self.name_by_id.len()
	}

	pub fn into_table(self) -> Vec<T> {
		self.name_by_id
	}

	pub fn get_id<B>(&self, name: &B) -> Option<&usize>
	where
		T: Borrow<B> + Ord,
		B: Ord + ?Sized,
	{
		self.id_by_name.get(name)
	}

	pub fn get_or_create_id(&mut self, name: T) -> usize
	where
		T: Clone + Ord,
	{
		match self.id_by_name.entry(name) {
			Entry::Vacant(entry) => {
				let next_id = self.name_by_id.len();
				self.name_by_id.push(entry.key().clone());
				entry.insert(next_id);
				next_id
			}
			Entry::Occupied(entry) => *entry.get(),
		}
	}
}
