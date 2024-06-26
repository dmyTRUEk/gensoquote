# gensoquote
Like [fortune](https://en.wikipedia.org/wiki/Fortune_(Unix)), but in Gensokyo and memory safe™.

[![gensoquote-git](https://img.shields.io/aur/version/gensoquote-git?color=1793d1&label=gensoquote-git&logo=arch-linux&style=for-the-badge)](https://aur.archlinux.org/packages/gensoquote-git/)

## Examples
```
"I'm the strongest!"
-- Cirno, "Phantasmagoria of Flower View"
```
```
"... She was supposed to be strong? Wait, she's your shikigami? Aren't you a shikigami yourself?"
-- Reimu Hakurei, "Perfect Cherry Blossom, Extra Stage"
```
```
"It ain't magic if it ain't flashy. Danmaku's all about firepower."
-- Marisa Kirisame, "Perfect Memento in Strict Sense"
```
```
"Read your mind? I've already closed my satori eye. Reading people's minds only makes you depressed, there's nothing good about it."
-- Koishi Komeiji to Reimu Hakurei, "Subterranean Animism, Extra Stage"
```

## Features
- Over 600 quotes from all over the Touhou Project
- Select your favorite character (`-c` or `--character` option)
- Format quote to any of your tastes (`-f` or `--format` option)

## Installation
### Using `cargo`
```
cargo install gensoquote
```

### From package manager
#### Arch Linux
Available as [AUR package](https://aur.archlinux.org/packages/gensoquote-git/) 🎉

### From source
#### Pre requirements: "Rust" installed
Install `rustup` using your preferred package manager
or follow the [official installation tutorial](https://www.rust-lang.org/tools/install).

#### Steps:
1. Clone the repo:
```
git clone https://github.com/dmyTRUEk/gensoquote
```

2. `cd` in it:
```
cd gensoquote
```

3. Build in release(optimized) mode:
```
cargo build --release
```

4. Locate single final binary at `./target/release/gensoquote`.
You can copy it to `~/.local/bin/` for convenient use.

## Usage
Simply run `gensoquote` to get random quote.

Run `gensoquote -h` or `gensoquote --help` to get help.

Select character who's random quote you want to get by running
`gensoquote -c koishi` or `gensoquote --character 'Koishi Komeiji'`.

Or pipe it to [fumosay](https://github.com/randomtwdude/fumosay) for even more funkyness:
`gensoquote | fumosay`.

Rumors say that piping it to [lolcat](https://github.com/busyloop/lolcat) generates unforgettable experience:
`gensoquote | fumosay | lolcat`.

## License
All the rights for quotes and characters belongs to corresponding author (mostly Zun),
while the code and related things are distributed under [MIT* license](./LICENSE.md).

## Contributions
Contributions are welcome!

Plz use tabs for indenting
(they have a lot of benefits compared to spaces, think about it).

## Acknowledgments
Thanks to [@randomtwdude](https://github.com/randomtwdude) and their [fumosay](https://github.com/randomtwdude/fumosay) for massive inspiration.

Thanks to [@kojq](https://github.com/kojq) for AUR packaging.
