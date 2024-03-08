# gensoquote
like [fortune](https://en.wikipedia.org/wiki/Fortune_(Unix)) but in Gensokyo and memory safe™.

## Installation
### From package manager
Currently no :'(

### From source
#### Pre requirements: "rust" installed
Install `rustup` (and then do `rustup toolchain install stable`) or simply `cargo`
from your preferred package manager
or follow the [official installation tutorial](https://www.rust-lang.org/tools/install).

1. Clone the repo:
```
git clone https://github.com/dmyTRUEk/gensoquote
```

2. `cd` in it:
```
cd gensoquote
```

3. Run or build in "debug"(not optimized) mode (or add `-r` or `--release` flag for "release"(optimized) version):
```
cargo run
```
or
```
cargo build
```

4. Locate single final binary at `./target/<debug or release>/gensoquote`.
You can copy it to `~/.local/bin/` to for convenient use.

## Usage
Simply run `gensoquote` to get random quote.

Or pipe it to [fumosay](https://github.com/randomtwdude/fumosay) for even more funkyness.

Rumors say that piping it to [lolcat](https://github.com/busyloop/lolcat) generates unforgettable experience.

## Contributions
Contributions are welcome!

Plz use tabs for indenting
(they have a lot of benefits compared to spaces).

## Acknowledgments
Thanks to @randomtwdude and their [fumosay](https://github.com/randomtwdude/fumosay) for massive inspiration.

