# common code for generate

def name_to_identifier(name: str) -> str:
	assert ',' not in name, f"`name` should not contains `,`, but it is: `{name}`"
	ident = (
		name
			.replace(' ', '_')
			.replace('-', '_')
			.replace("'", '')
			.replace('.', "_")
			.replace('Σ', "Sigma")
	)
	assert is_ascii_numeric(ident), f"`ident` must be alpha-numeric, but it is: `{ident}`"
	return ident

def is_ascii_numeric(ident: str) -> bool:
	return ident[0].isascii() and all(map(lambda c: c.isascii() or c.isnumeric(), ident))
