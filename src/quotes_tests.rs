//! Tests for `quotes.rs`

#[cfg(test)]
mod tests {
	use crate::{quote::Quote, quotes::QUOTES};

	#[test]
	fn dont_contain_unwanted_text() {
		const UNWANTED: &[&str] = &[
			"text:",
			"char:",
			"src:",
			"whom_to",
			"whom_about",
			"\n",
			"\t",
		];
		for Quote { text, src, .. } in QUOTES {
			for unwanted in UNWANTED {
				assert!(!text.to_lowercase().contains(unwanted), "unwanted=`{unwanted}`, text=`{text}`");
				assert!(!src.to_lowercase().contains(unwanted), "unwanted=`{unwanted}`, src=`{src}`");
			}
		}
	}
}
