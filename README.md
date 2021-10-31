# palscii

A simple command line tool for creating font palettes for engines like libtcod.

## Usage

_This can also be viewed by running `palscii --help`_.

```
palscii - generate ASCII font palettes for rougelike engines

palscii is designed to be a dead-simple UNIX-like tool. It takes font files and makes PNG palettes,
and it aims to do that well.

### NOTES

The glyph parameters `width` and `height` are scaled to on a best-case scenario. In situations where
it is not possible to do so, cropping will occur and may result in undesirable visual artifacts.

In DEBUG builds of palscii, an assertion checks the validity of the scale operation, so palscii will
exit with an error.

USAGE:
    palscii [OPTIONS]

OPTIONS:
    -h, --height <HEIGHT>
            Optional maximum glyph height, defualt: 16. See **Notes** for more details

            [default: 16]

        --help
            Print help information

    -i, --input <INPUT>
            Optional path to input file, if not provided, STDIN will be used

    -o, --output <OUTPUT>
            Optional path to output to, if not provided, STDOUT will be used

    -w, --width <WIDTH>
            Optional maximum glyph width, default: 8. See **Notes** for more details

            [default: 8]
```

## Building

TL;DR: `cargo build` or `cargo build --release` and you should be up-and-running.

This is a Rust project so it requries a Rust toolchain.
See the official [Rust website](https://www.rust-lang.org/tools/install) for getting that set up.

After that, it should be as easy as:

- `git clone` the repo
- `cargo build` to build, if you don't want debug assertions, `cargo build --release`
- outputs go into `target/`; the `palscii` bin is self-contained and portable

I developed and tested exclusively the following environment:

- MacOs version `11.6` (Big Sur)
- `rustc` version `1.54.0`
- `cargo` version `1.54.0`

I don't anticipate that Palscii would have any problems running on Linux distros,
but Windows users might run into some quirks due to different handling of paths.

## Testing

- `cargo test`

If you'd like to manually test, I've included a couple OFL-licensed fonts along
with their respective attribution and license text under `resources/`.

## About

Palscii, at a high level, takes an OTF or TTF font, and creates a 16x16 PNG
palette for use in rougelike game engines such as libtcod or bracket-lib.

I created Palscii over a weekend + a few hours here and there in order to play
with different fonts while reading Herbert Wolverson's Rougelike in Rust book.
That book can be found [here](http://bfnightly.bracketproductions.com/chapter_0.html)
and I highly recommend it for anyone interested in Rust and rougelike games.

I designed Palscii to be a simple tool following the UNIX philosophy; namely,
it does _one thing_ and tries to do it well.
Since this was also a total hack project that I made for fun (and to support my
main fun project which is making a rougelike), it's still pretty rough around
the edges in v0.1.0.
If something doesn't quite work the way you'd like, feel free to fork me and/or
PR a fix here.
