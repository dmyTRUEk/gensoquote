//! Quote struct.

use crate::characters::Character;

pub struct Quote {
	pub text: &'static str,
	pub char: Character,
	pub src: &'static str,
	pub whom_to: Option<Character>,
	/// `char` says `text` about who?
	pub whom_about: Option<Character>,
}

impl Quote {
	pub const fn default() -> Self {
		Self {
			text: "default text",
			char: Character::Unknown,
			src: "default src",
			whom_to: None,
			whom_about: None,
		}
	}
}
