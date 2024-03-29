//! Main

use clap::{arg, Parser};
use rand::{thread_rng, Rng};

// mod artbooks;
mod characters;
// mod games;
mod quote;
mod quotes;
// mod source;
mod to_str;

use crate::{quote::Quote, quotes::QUOTES, to_str::ToStr};

#[derive(Parser, Debug)]
#[command(about, version, long_about = None, author)]
struct CliArgs {
	/// Name of the character
	#[arg(short, long)]
	character: Option<String>,
}

fn main() -> Result<(), &'static str> {
	let cli_args = CliArgs::parse();
	let Quote { text, char, src, whom_to, whom_about } = get_random_quote(cli_args.character)?;
	let char = char.to_str();
	let maybe_to = whom_to
		.map(|whom_to| format!(" to {}", whom_to.to_str()))
		.unwrap_or_default();
	let maybe_about = whom_about
		.map(|whom_about| format!(" about {}", whom_about.to_str()))
		.unwrap_or_default();
	println!("\"{text}\"\n-- {char}{maybe_to}{maybe_about}, \"{src}\"");
	Ok(())
}

fn get_random_quote(char: Option<String>) -> Result<&'static Quote, &'static str> {
	let mut rng = thread_rng();
	match char {
		None => {
			let random_quote_index: usize = rng.gen_range(0..QUOTES.len());
			Ok(&QUOTES[random_quote_index])
		}
		Some(char) => {
			let char_lowercase: String = char.to_lowercase();
			let quotes: Vec<&Quote> = QUOTES.iter()
				.filter(|quote| quote.char.to_str().to_lowercase().contains(&char_lowercase))
				.collect();
			if quotes.len() == 0 { return Err("Character's quote not found.") }
			let random_quote_index: usize = rng.gen_range(0..quotes.len());
			Ok(quotes[random_quote_index])
		}
	}
}
