//! ALL touhou quotes.

use crate::{
	artbooks::Artbook::*,
	characters::Character::*,
	games::Game::*,
	quote::Quote,
	source::Source::*,
};

pub const QUOTES: &[Quote] = &[
	Quote {
		text: "Out of the generations of shrine maidens, her sense of danger is the most lacking and she has meager training, yet her power is considerable.",
		char: Hieda_no_Akyuu,
		src: Some(Artbook(Perfect_Memento_in_Strict_Sense)),
		whom_about: Some(Reimu_Hakurei),
	},
	Quote {
		text: "It ain't magic if it ain't flashy. Danmaku's all about firepower.",
		char: Marisa_Kirisame,
		src: Some(Artbook(Perfect_Memento_in_Strict_Sense)),
		..Quote::default()
	},
];
