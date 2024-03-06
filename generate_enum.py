# generates rust enum file

import os
import sys

def main():
	filepath_input = sys.argv[1]
	filename_input = os.path.basename(filepath_input)
	assert filename_input.endswith(".txt"), "input file name must end with `.txt`"
	enum_name_lowercase = filename_input[:-4]
	assert enum_name_lowercase.endswith('s'), f"`enum_name_lowercase` should end with `s`, but it is: `{enum_name_lowercase}`"
	enum_name_camelcase = enum_name_lowercase[0].upper() + enum_name_lowercase[1:-1]
	filename_output = "src/" + enum_name_lowercase + ".rs"

	output = []
	with open(filepath_input, 'r') as file_input:
		file_input_lines = file_input.readlines()
		lines = map(str.strip, file_input_lines)
		src = next(lines)
		assert src.startswith("src: "), f"first line must contain `src: ...`, but it is: `{src}`"
		assert next(lines) == "", "second line mustn't contain any information"
		output.append(f"//! Enum of all official Touhou {enum_name_lowercase}.")
		output.append( "//!")
		output.append(f"//! {src}")
		output.append("")
		output.append("// DO NOT EDIT THIS FILE BY HANDS!")
		output.append("// INSTEAD EDIT CORRESPONDING FILE!")
		output.append("")
		output.append("use crate::to_str::ToStr;")
		output.append("")
		output.append("#[allow(non_camel_case_types, dead_code)]")
		output.append("#[derive(Debug)]")
		output.append(f"pub enum {enum_name_camelcase} {{")
		for line in lines:
			if line.startswith('//'):
				output.append('\t' + line)
				continue
			elif line == "":
				output.append("")
				continue
			name = line
			assert ',' not in name, f"name should not contains `,`, but it is: `{name}`"
			ident = name_to_identifier(name)
			assert is_ascii_numeric(ident), f"`ident` must be alpha-numeric, but it is: `{ident}`"
			output.append('\t' + ident + ',')
		output.append(f"}}")
		output.append("")
		# TODO: write "impl "
		output.append(f"impl ToStr for {enum_name_camelcase} {{")
		output.append("\tfn to_str(&self) -> &str {")
		output.append("\t\tmatch self {")
		lines = map(str.strip, file_input_lines)
		next(lines); next(lines)
		for line in lines:
			if line.startswith('//'):
				output.append('\t'*3 + line)
				continue
			elif line == "":
				output.append("")
				continue
			name = line
			ident = name_to_identifier(name)
			assert is_ascii_numeric(ident), f"`ident` must be alpha-numeric, but it is: `{ident}`"
			output.append('\t'*3 + f'Self::{ident} => "{name}",')
		output.append("\t\t}")
		output.append("\t}")
		output.append(f"}}")

	with open(filename_output, 'w') as file_output:
		file_output.write('\n'.join(output) + '\n')



def name_to_identifier(text: str) -> str:
	return (
		text
			.replace(' ', '_')
			.replace('-', '_')
			.replace("'", '')
			.replace('Σ', "Sigma")
			.replace('.', "_")
	)


def is_ascii_numeric(ident: str) -> bool:
	return ident[0].isascii() and all(map(lambda c: c.isascii() or c.isnumeric(), ident))


if __name__ == "__main__":
	main()
