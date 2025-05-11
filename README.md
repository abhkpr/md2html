# md2html
Rust command-line tool that converts Markdown to HTML, intended for use on Linux terminals.

## md2html

A blazing-fast, minimal Rust command-line tool to convert Markdown (`.md`) files into HTML.  
Built with [`pulldown-cmark`](https://crates.io/crates/pulldown-cmark) and [`clap`](https://crates.io/crates/clap) for optimal speed and ergonomics.

---

## Features

- Converts Markdown to clean HTML
- Output to file or stdout
- Lightweight and fast
- Easy to integrate in scripts or automation

---

## Installation

### Clone and build:

```bash
git clone https://github.com/yourusername/md2html.git
cd md2html
cargo build --release

```
## Usage
### Convert markdown and save to HTML file
- ./target/release/md2html input.md -o output.html

### Print HTML to terminal
- ./target/release/md2html input.md


