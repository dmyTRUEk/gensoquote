//! Quote's source (game, book, etc).

use crate::{games::Game, artbooks::Artbook, to_str::ToStr};

#[derive(Debug)]
pub enum Source {
	Game(Game),
	Artbook(Artbook),
}

impl ToStr for Source {
	fn to_str(&self) -> &str {
	    match self {
	        Self::Game(game) => game.to_str(),
	        Self::Artbook(artbook) => artbook.to_str(),
	    }
	}
}
