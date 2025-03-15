# Whodunnit

An experimental game approach to exploring Prolog through Rust and solving murder mysteries.

## Requirements

You need to have both Rust and Prolog installed.

To install Rust follow the [official language guide](https://www.rust-lang.org/tools/install).

I only tested this setup with SWI-Prolog so I recommend installing that one. You can find [binaries on the official website](https://www.swi-prolog.org/download/stable). After downloading/ installing, make sure `swipl` is available in your path by running `$ swipl --version` in your terminal.

## Running it

After installing Rust and Prolog, download or clone this repository and cd into `whodunnit` (this) directory.

Then run `$ cargo run`. Cargo will download and build all dependencies and run the project.

In case it complains about file paths you might need to add the `tmp` folder manually, like `$ mkdir tmp` on the same level as `src` and `game`.

That's it!

(Also note that the actual game logic does not work, you can [read about it here](https://lislis.github.io/whodunnit/blog/results/)).
