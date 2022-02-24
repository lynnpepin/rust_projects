# Rust Notes

> **TLDR: These are my notes. I'm using these resources.**

These are my condensed and cool notes for the Rust programming language, written from the perspective of a Python programmer. Sources used throughout are:

 - [The Rust Book](https://doc.rust-lang.org/stable/book/)
 - [The Cargo Book](https://doc.rust-lang.org/cargo/)
 - [The Rustlings Course](https://github.com/rust-lang/rustlings/) to get you started with tooling.
 - [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/) which provides an in-browser Rust interpreter.
    - Play in [the Rust Playground at play.rust-lang.org](https://play.rust-lang.org/).
 - [Learn Rust in Y Minutes](https://learnxinyminutes.com/docs/rust/)

Rust also provides their documentation offline, e.g. `rustup --docs book`:

| `rustup --docs ...` | Gives you | 
| ----  | ----------------------- |
| `book`  | *The* Rust Book.
| `std`   | Standard Library. Builds on the Core.
| `rust-by-example` |
| `cargo` | Using Cargo

Rust also has some more advanced docs, such as the `reference`, the `core` docs, the `nomicon` for dark forebidden rust, or `embedded-book` for embedded programming. Get the full list with `rustup docs -h`.



---

## First things first: Install and Cargo

> **TLDR: Use `cargo` from the start.**

[Install Rustup](https://www.rust-lang.org/learn/get-started). It'll also give you **Cargo**, the build-tool and package manager. It is the Conda for your Rust.

The right way to use Rust is to use **Cargo**, and the right way to manage external dependencies is the `Cargo.toml` file, which automatically talks to [crates.io](https://crates.io/).

For those coming from Python, compare:

| Rust       | Python | 
|------------|--------|
| Cargo      | Conda or Pip
| Cargo.toml | setup.cfg, envs, and requirements.txt
| crates.io  | PyPi, Conda Forge
| crates     | Wheels / eggs

Here are some Cargo tools you'll use regularly:

- `cargo new some_lib_name --lib`
    - Will create a new project, instantiating `Cargo.toml` and `src/lib.rs`.
    - Omit `--lib` if you want to make an executable program instead of a library.
- `cargo test`
    - Will run tests, which can be included inline your file.
    - Good thing `lib.rs` gives you these by default!
- `cargo run`
    - Will run your `main` function under `main.rs` if you have it.
    - `cargo run --release -- {put args here}` will build with optimization. Put your command-line arguments after `--`.
- See also: `cargo doc`, `cargo build`, `cargo publish`, etc.

If you **just want a tool like `gcc` for Rust, use `rustc`**. Once you need to `use` something outside the stdlib, use `cargo`.

### Do things right first.

Rust starts you with nice things that usually come later: Tests, docs, managing dependencies, and readiness to publish.

Let's keep you on the right foot. You probably want to structure your projects right.

1. [Structuring your projects](https://doc.rust-lang.org/cargo/guide/project-layout.html)

2. Argument Parsing: [(1) Tutorial from rust-cli](https://rust-cli.github.io/book/tutorial/cli-args.html), [(2) CLAP on doc.rs](https://docs.rs/clap/2.33.0/clap/), and a variety of [(3) great and simple examples](https://github.com/clap-rs/clap/tree/master/examples) to get you started.

3. [Writing documentation](https://doc.rust-lang.org/rust-by-example/meta/doc.html). (Covers syntax, not convention.)

4. Most of the time, all you need are `vec!`, `std::collections::HashMap`, primitives and strings,  and slices.

---

# Rust Book: The Language parts

A crash-course through the chapters in the book dealing with the language and it's std-lib.

These are chapters 3, 4, 5, 6, 8, 9, 10, 18, and the Appendix.

Chapters 2, 7, 11, 12, 14, 20 deal with hands-on examples and project management.

Chapters 13, 15, 16, 17, 18, 19 deal with advanced language features.