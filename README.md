# Minigrep

## Requirements
[Rust-lang](https://www.rust-lang.org/en-US/install.html) should be installed in
your system.

## Installation
```sh
git clone https://github.com/subhojit777/minigrep.git
cd minigrep
cargo build --release
```

## Usage
```sh
./target/release/minigrep <options> nemo find-nemo-the-movie.txt
```

### Allowed Options
```
i - Case-insensitive.
w - Exact match.
```

## Documentation
```sh
cargo doc --no-deps --open
```

## Run tests
```sh
cargo test
```

## Note
This is not a drop in replacement for the good old grep-like commands. This is
written for the sake of learning Rust-lang.

## Motivation behind open sourcing
Actually this is an excersice in [the book](https://doc.rust-lang.org/book/second-edition/ch12-00-an-io-project.html).
I have open sourced this because I have followed a different implementation than
what is suggested in the book. [TDD](https://en.wikipedia.org/wiki/Test-driven_development)
followed while writing the helper libraries.

Feedbacks are welcome :)
