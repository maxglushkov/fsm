use std::ops;

#[derive(Clone, Copy, Default)]
pub struct StateType(u8);

impl StateType {
	pub const INITIAL: StateType = StateType(1);
	pub const FINAL: StateType = StateType(2);
	const NAMES: [char; 4] = ['M', 'S', 'F', 'X'];

	pub fn from_last_char(name: &str) -> Self {
		let name = name.chars().next_back().unwrap_or_default();
		StateType(
			Self::NAMES
				.iter()
				.position(|&n| n == name)
				.unwrap_or_default() as u8,
		)
	}

	pub fn name(self) -> char {
		StateType::NAMES[self.0 as usize]
	}
}

impl ops::BitAnd for StateType {
	type Output = bool;

	fn bitand(self, rhs: Self) -> Self::Output {
		self.0 & rhs.0 != 0
	}
}

impl ops::BitAndAssign for StateType {
	fn bitand_assign(&mut self, rhs: Self) {
		self.0 &= rhs.0;
	}
}

impl ops::BitOrAssign for StateType {
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0;
	}
}
