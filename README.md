# 🦀 Minigrep (Rust CLI Project)

A simple command-line search tool written in Rust, inspired by the “minigrep” project from the Rust Book.

This project searches for a query string within a file and prints matching lines, similar to a very small subset of `grep`.

---

## 📚 Background

This implementation follows the Rust Book’s CLI project and includes improvements such as:

- Clean separation of logic (`lib.rs`) and CLI handling (`main.rs`)
- Proper error handling using `Result`
- Writing error messages to **standard error (`stderr`) instead of standard output (`stdout`)**

In command-line programs, errors should go to `stderr` so users can redirect normal output without losing error messages.

---

## ✨ Features

- 🔍 Search for a string in a file
- 🔡 Optional case-insensitive search
- ⚙️ Environment variable support (`IGNORE_CASE`)
- ⚠️ Proper error reporting via `stderr`
- 🧩 Modular and testable design

---

## 🚀 Usage

### Basic Usage

```bash
cargo run -- <query> <file_path>

Example:

cargo run -- duct poem.txt
Case-Insensitive Search

Set the IGNORE_CASE environment variable:

IGNORE_CASE=1 cargo run -- duct poem.txt
📂 Project Structure
minigrep/
├── src/
│   ├── main.rs     # CLI entry point
│   ├── lib.rs      # Core logic (search functionality)
├── Cargo.toml
└── README.md
```
## ⚙️ How It Works
1. Parse command-line arguments
2. Build a configuration struct
3. Run the search logic
4. Print matching lines to stdout
5. Print errors to stderr using eprintln!

Example error handling:
```bash
eprintln!("Problem parsing arguments: {err}");
process::exit(1);
```

## 🔄 Output Behavior

This project correctly separates output streams:

✅ Search results → stdout
❌ Errors → stderr

This allows useful shell behavior like:

```bash
cargo run -- query file.txt > output.txt
Matches go into output.txt
Errors still appear in the terminal
```

## 🧪 Testing

Run tests with:

```bash
cargo test
```
Tests are implemented for:

-  Case-sensitive search
-  Case-insensitive search

## 📜 License
MIT
