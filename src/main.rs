//! Main

use clap::{arg, Parser};
use rand::{thread_rng, Rng};

mod characters;
mod format;
mod quote;
mod quotes;
mod to_str;

use crate::{
	format::{FORMATTING_DEFAULT, format_quote},
	quote::Quote,
	quotes::QUOTES,
	to_str::ToStr,
};


#[derive(Parser, Debug)]
#[clap(
	about,
	author,
	version,
	help_template = "\
		{before-help}{name} {version}\n\
		{about}\n\
		Author: {author}\n\
		\n\
		{usage-heading} {usage}\n\
		\n\
		{all-args}{after-help}\
	",
)]
struct CliArgs {
	/// Character whose random quote to select
	#[arg(short, long)]
	character: Option<String>,

	/// Format string
	///
	/// Formatters:
	/// %q - quote (text)
	/// %c - character
	/// %s - source
	/// (%t) - whom to
	/// (%a) - whom about
	///
	/// Optional formatter must be enclosed in round brackets,
	/// which may have additional text inside.
	///
	/// Example:
	/// `"%q" -- %c( says to %t), %s`
	/// if %t is none:
	/// `"My hat is my friend." -- Koishi Komeiji, KKHTA`
	/// if %t is not none:
	/// `"My hat is my friend." -- Koishi Komeiji says to Koishi Komeiji, KKHTA`
	#[arg(short, long, verbatim_doc_comment, default_value_t={FORMATTING_DEFAULT.to_string()})]
	format: String,
}


fn main() -> Result<(), &'static str> {
	let cli_args = CliArgs::parse();
	let quote = get_random_quote(cli_args.character)?;
	let quote_formatted = format_quote(quote, &cli_args.format);
	println!("{quote_formatted}");
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
