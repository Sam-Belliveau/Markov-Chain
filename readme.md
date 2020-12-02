# Markov Chain Generator

This is a project paid for by [**Qiong Zhou Huang**](www.mit.edu) as an attempt to generate LMFAO statements similar to his friend **Endar**

## How to install

### Arch Linux
To install this package on arch linux, run:
```sh
wget https://github.com/Sam-Belliveau/Markov-Chain/releases/download/0.2.0/markov_chain-bin-0.2.0-1-x86_64.pkg.tar.xz
sudo pacman -U ./markov_chain-bin-0.2.0-1-x86_64.pkg.tar.xz
```

Then you should be able to run by running `> markov_chain`

## How To Compile

You're going to need Rust to compile this. Just CD into the repository, and run `cargo run --release -- --help` for the help page. You need to use `cargo run --` in order to use any of the other command line options.

using `--release` is 100% worth it as it makes generating markov chains based on large data sets orders of magnitude faster.

## How to use

When you run the `--help` command, it spits out this:

```bash
Sam's Markov Chain Generator 0.1.0
Sam Belliveau <sam.belliveau@gmail.com>
Generate text based on the character probabilities from a given dictionary.

USAGE:
    markov_chain [OPTIONS] --dict <dictionary>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --chain-size <chain size>          How many past characters to consider when building probabilites [default: 2]
    -d, --dictionary <dictionary>          Dictionary from which character probabilities are built from
    -o, --output-length <output length>    The length of the output Markov Chain [default: 1000]
```

So Qiong, I know you're the only one using this, so just type `cargo run -- -d endar_lmaos.txt`.
