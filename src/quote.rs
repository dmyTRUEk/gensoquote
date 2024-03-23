//! Quote struct.

use crate::characters::Character;

pub struct Quote {
	pub text: &'static str,
	pub char: Character,
	pub src: Option<&'static str>,
	/// `char` says `text` about who?
	pub whom_about: Option<Character>,
	pub whom_to: Option<Character>,
}

impl Quote {
	pub const fn default() -> Self {
		Self {
			text: "default text",
			char: Character::Unknown,
			src: None,
			whom_about: None,
			whom_to: None,
		}
	}
}
