# Markov Chain Generator

This is a project paid for by [**Qiong Zhou Huang**](www.mit.edu) as an attempt to generate LMFAO statements similar to his friend **Endar**

## How To Compile

You're going to need Rust to compile this. Just CD into the repository, and run `cargo run -- --help` for the help page. You need to use `cargo run --` in order to use any of the other command line options.

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
