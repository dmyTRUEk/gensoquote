//! Format quote by %q and other.

use crate::{quote::Quote, to_str::ToStr};


pub const FORMATTING_DEFAULT: &str = "\"%q\"\n-- %c( to %t)( about %a), \"%s\"";

const FORMAT_CHAR_TEXT: char = 'q';
const FORMAT_CHAR_CHAR: char = 'c';
const FORMAT_CHAR_SRC : char = 's';
const FORMAT_CHAR_WHOM_TO: char = 't';
const FORMAT_CHAR_WHOM_ABOUT: char = 'a';
const BRACKET_LEFT: char = '(';
const BRACKET_RIGHT: char = ')';


pub fn format_quote(quote: &Quote, format: &str) -> String {
	let Quote { text, char, src, whom_to, whom_about } = quote;

	let mut quote_formatted: String = format
		.replace(&get_formatter(FORMAT_CHAR_TEXT), text)
		.replace(&get_formatter(FORMAT_CHAR_CHAR), char.to_str())
		.replace(&get_formatter(FORMAT_CHAR_SRC), src);

	quote_formatted = format_optional(
		&quote_formatted,
		FORMAT_CHAR_WHOM_TO,
		whom_to.as_ref().map(|whom_to| whom_to.to_str().to_string()),
	);

	quote_formatted = format_optional(
		&quote_formatted,
		FORMAT_CHAR_WHOM_ABOUT,
		whom_about.as_ref().map(|whom_about| whom_about.to_str().to_string()),
	);

	quote_formatted = quote_formatted.replace("\\n", "\n");

	quote_formatted
}


fn get_formatter(c: char) -> String {
	format!("%{c}")
}

fn format_optional(
	quote_formatted: &str,
	format_char: char,
	replace_formatter_by: Option<String>,
) -> String {
	let maybe_amp_pos: Option<usize> = quote_formatted
		.find(&get_formatter(format_char));
	let amp_pos: usize = match maybe_amp_pos {
		None => return quote_formatted.to_string(),
		Some(amp_pos) => amp_pos,
	};

	let maybe_l_bracket_pos: Option<usize> = quote_formatted[..amp_pos]
		.char_indices()
		.rev()
		.find(|(_i, c)| *c == BRACKET_LEFT)
		.map(|(i, _c)| i);

	let maybe_r_bracket_pos: Option<usize> = quote_formatted[amp_pos..]
		.char_indices()
		.find(|(_i, c)| *c == BRACKET_RIGHT)
		.map(|(i, _c)| i)
		.map(|i| i + amp_pos);

	match (maybe_l_bracket_pos, maybe_r_bracket_pos) {
		(Some(l_bracket_pos), Some(r_bracket_pos)) => {
			let replace_formatting_by: String = match replace_formatter_by {
				Some(replace_formatter_by) => {
					quote_formatted[l_bracket_pos+1..=r_bracket_pos-1]
						.replace(
							&get_formatter(format_char),
							&replace_formatter_by
						)
				}
				None => String::new(),
			};
			[
				&quote_formatted[..l_bracket_pos],
				&replace_formatting_by,
				&quote_formatted[r_bracket_pos+1..],
			].concat()
		}
		_ => quote_formatted.to_string()
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	mod default_formatting {
		use super::*;

		#[test]
		fn without_to_without_about() {
			use crate::characters::Character::Marisa_Kirisame;
			let quote = Quote {
				text: "It ain't magic if it ain't flashy. Danmaku's all about firepower.",
				char: Marisa_Kirisame,
				src: "Perfect Memento in Strict Sense",
				..Quote::default()
			};
			assert_eq!(
				String::from("\
					\"It ain't magic if it ain't flashy. Danmaku's all about firepower.\"\n\
					-- Marisa Kirisame, \"Perfect Memento in Strict Sense\"\
				"),
				format_quote(&quote, FORMATTING_DEFAULT),
			);
		}

		#[test]
		fn with_to_without_about() {
			use crate::characters::Character::{Shinki, Yuuka_Kazami};
			let quote = Quote {
				text: "Massacres are a kind of game, too. It doesn't matter whether it's humans or Makai residents",
				char: Yuuka_Kazami,
				src: "Mystic Square",
				whom_to: Some(Shinki),
				..Quote::default()
			};
			assert_eq!(
				String::from("\
					\"Massacres are a kind of game, too. It doesn't matter whether it's humans or Makai residents\"\n\
					-- Yuuka Kazami to Shinki, \"Mystic Square\"\
				"),
				format_quote(&quote, FORMATTING_DEFAULT),
			);
		}

		#[test]
		fn without_to_with_about() {
			use crate::characters::Character::{Hieda_no_Akyuu, Reimu_Hakurei};
			let quote = Quote {
				text: "Out of the generations of shrine maidens, her sense of danger is the most lacking and she has meager training, yet her power is considerable.",
				char: Hieda_no_Akyuu,
				src: "Perfect Memento in Strict Sense",
				whom_about: Some(Reimu_Hakurei),
				..Quote::default()
			};
			assert_eq!(
				String::from("\
					\"Out of the generations of shrine maidens, her sense of danger is the most lacking and she has meager training, yet her power is considerable.\"\n\
					-- Hieda no Akyuu about Reimu Hakurei, \"Perfect Memento in Strict Sense\"\
				"),
				format_quote(&quote, FORMATTING_DEFAULT),
			);
		}

		#[test]
		fn with_to_with_about() {
			use crate::characters::Character::{Shinki, Yumeko, Yuuka_Kazami};
			let quote = Quote {
				text: "You musn't dirty your hands dealing with this kind of person! I shall deal with her promptly, so please, step back, Lady Shinki.",
				char: Yumeko,
				src: "Mystic Square, Stage 5",
				whom_to: Some(Shinki),
				whom_about: Some(Yuuka_Kazami),
			};
			assert_eq!(
				String::from("\
					\"You musn't dirty your hands dealing with this kind of person! I shall deal with her promptly, so please, step back, Lady Shinki.\"\n\
					-- Yumeko to Shinki about Yuuka Kazami, \"Mystic Square, Stage 5\"\
				"),
				format_quote(&quote, FORMATTING_DEFAULT),
			);
		}
	}

	mod optional {
		use super::*;
		use crate::characters::Character::Cirno;

		mod single_byte {
			use super::*;

			mod none {
				use super::*;
				const QUOTE: Quote = Quote {
					whom_to: None,
					..Quote::default()
				};

				#[test]
				fn only_formatter() {
					assert_eq!(
						String::from(""),
						format_quote(&QUOTE, "(%t)"),
					);
				}

				#[test]
				fn text_outside_on_left() {
					assert_eq!(
						String::from("abc"),
						format_quote(&QUOTE, "abc(%t)"),
					);
				}

				#[test]
				fn text_outside_on_right() {
					assert_eq!(
						String::from("abc"),
						format_quote(&QUOTE, "(%t)abc"),
					);
				}

				#[test]
				fn surrounded_outside() {
					assert_eq!(
						String::from("abcdef"),
						format_quote(&QUOTE, "abc(%t)def"),
					);
				}

				#[test]
				fn text_inside_on_left() {
					assert_eq!(
						String::from(""),
						format_quote(&QUOTE, "(abc%t)"),
					);
				}

				#[test]
				fn text_inside_on_right() {
					assert_eq!(
						String::from(""),
						format_quote(&QUOTE, "(%tabc)"),
					);
				}

				#[test]
				fn surrounded_inside() {
					assert_eq!(
						String::from(""),
						format_quote(&QUOTE, "(abc%tdef)"),
					);
				}

				#[test]
				fn surrounded() {
					assert_eq!(
						String::from("abcjkl"),
						format_quote(&QUOTE, "abc(def%tghi)jkl"),
					);
				}
			}

			mod some {
				use super::*;
				const QUOTE: Quote = Quote {
					whom_to: Some(Cirno),
					..Quote::default()
				};

				#[test]
				fn only_formatter() {
					assert_eq!(
						String::from("Cirno"),
						format_quote(&QUOTE, "(%t)"),
					);
				}

				#[test]
				fn text_outside_on_left() {
					assert_eq!(
						String::from("abcCirno"),
						format_quote(&QUOTE, "abc(%t)"),
					);
				}

				#[test]
				fn text_outside_on_right() {
					assert_eq!(
						String::from("Cirnoabc"),
						format_quote(&QUOTE, "(%t)abc"),
					);
				}

				#[test]
				fn surrounded_outside() {
					assert_eq!(
						String::from("abcCirnodef"),
						format_quote(&QUOTE, "abc(%t)def"),
					);
				}

				#[test]
				fn text_inside_on_left() {
					assert_eq!(
						String::from("abcCirno"),
						format_quote(&QUOTE, "(abc%t)"),
					);
				}

				#[test]
				fn text_inside_on_right() {
					assert_eq!(
						String::from("Cirnoabc"),
						format_quote(&QUOTE, "(%tabc)"),
					);
				}

				#[test]
				fn surrounded_inside() {
					assert_eq!(
						String::from("abcCirnodef"),
						format_quote(&QUOTE, "(abc%tdef)"),
					);
				}

				#[test]
				fn surrounded() {
					assert_eq!(
						String::from("abcdefCirnoghijkl"),
						format_quote(&QUOTE, "abc(def%tghi)jkl"),
					);
				}
			}
		}

		mod multi_byte {
			use super::*;

			mod none {
				use super::*;
				const QUOTE: Quote = Quote {
					whom_to: None,
					..Quote::default()
				};

				#[test]
				fn only_formatter() {
					assert_eq!(
						String::from(""),
						format_quote(&QUOTE, "(%t)"),
					);
				}

				#[test]
				fn text_outside_on_left() {
					assert_eq!(
						String::from("東"),
						format_quote(&QUOTE, "東(%t)"),
					);
				}

				#[test]
				fn text_outside_on_right() {
					assert_eq!(
						String::from("東"),
						format_quote(&QUOTE, "(%t)東"),
					);
				}

				#[test]
				fn surrounded_outside() {
					assert_eq!(
						String::from("東方"),
						format_quote(&QUOTE, "東(%t)方"),
					);
				}

				#[test]
				fn text_inside_on_left() {
					assert_eq!(
						String::from(""),
						format_quote(&QUOTE, "(東%t)"),
					);
				}

				#[test]
				fn text_inside_on_right() {
					assert_eq!(
						String::from(""),
						format_quote(&QUOTE, "(%t東)"),
					);
				}

				#[test]
				fn surrounded_inside() {
					assert_eq!(
						String::from(""),
						format_quote(&QUOTE, "(東%t方)"),
					);
				}

				#[test]
				fn surrounded() {
					assert_eq!(
						String::from("東石"),
						format_quote(&QUOTE, "東(方%t小)石"),
					);
				}
			}

			mod some {
				use super::*;
				const QUOTE: Quote = Quote {
					whom_to: Some(Cirno),
					..Quote::default()
				};

				#[test]
				fn only_formatter() {
					assert_eq!(
						String::from("Cirno"),
						format_quote(&QUOTE, "(%t)"),
					);
				}

				#[test]
				fn text_outside_on_left() {
					assert_eq!(
						String::from("東Cirno"),
						format_quote(&QUOTE, "東(%t)"),
					);
				}

				#[test]
				fn text_outside_on_right() {
					assert_eq!(
						String::from("Cirno東"),
						format_quote(&QUOTE, "(%t)東"),
					);
				}

				#[test]
				fn surrounded_outside() {
					assert_eq!(
						String::from("東Cirno方"),
						format_quote(&QUOTE, "東(%t)方"),
					);
				}

				#[test]
				fn text_inside_on_left() {
					assert_eq!(
						String::from("東Cirno"),
						format_quote(&QUOTE, "(東%t)"),
					);
				}

				#[test]
				fn text_inside_on_right() {
					assert_eq!(
						String::from("Cirno東"),
						format_quote(&QUOTE, "(%t東)"),
					);
				}

				#[test]
				fn surrounded_inside() {
					assert_eq!(
						String::from("東Cirno方"),
						format_quote(&QUOTE, "(東%t方)"),
					);
				}

				#[test]
				fn surrounded() {
					assert_eq!(
						String::from("東方Cirno小石"),
						format_quote(&QUOTE, "東(方%t小)石"),
					);
				}
			}
		}

		#[ignore]
		#[test]
		fn escaped_bracket_left() {
			todo!()
		}
		#[ignore]
		#[test]
		fn escaped_bracket_right() {
			todo!()
		}
	}
}
