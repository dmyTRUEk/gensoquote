//! Main

use rand::{thread_rng, Rng};

// mod artbooks;
mod characters;
// mod games;
mod quote;
mod quotes;
// mod source;
mod to_str;

use crate::{quote::Quote, quotes::QUOTES, to_str::ToStr};

fn main() {
	let random_quote_index: usize = get_random_quote_index();
	let Quote { text, char, src, whom_about, whom_to } = &QUOTES[random_quote_index];
	let char: &'static str = char.to_str();
	let maybe_to = if let Some(whom_to) = whom_to { format!(" to {}", whom_to.to_str()) } else { format!("") };
	let maybe_about = if let Some(whom_about) = whom_about { format!(" about {}", whom_about.to_str()) } else { format!("") };
	let maybe_src = if let Some(src) = src { format!(", \"{}\"", src)} else { format!("") };
	println!("\"{text}\"\n-- {char}{maybe_to}{maybe_about}{maybe_src}");
}

fn get_random_quote_index() -> usize {
	let mut rng = thread_rng();
	rng.gen_range(0..QUOTES.len())
}
