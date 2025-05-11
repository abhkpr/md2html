use std::fs;
use std::path::Path;
use std::process;

use clap::Parser;
use pulldown_cmark::{Parser as MdParser, Options, html};

/// Convert a Markdown file to HTML
#[derive(Parser)]
#[command(name = "md2html")]
#[command(version = "1.0")]
#[command(about = "Convert Markdown to HTML in terminal", long_about = None)]
struct Args {
    /// Input markdown file
    input: String,

    /// Optional output HTML file
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input_path = Path::new(&args.input);
    let content = fs::read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("❌ Failed to read {}: {}", args.input, err);
        process::exit(1);
    });

    let parser = MdParser::new_ext(&content, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    match &args.output {
        Some(output_file) => {
            if let Err(e) = fs::write(output_file, html_output) {
                eprintln!("❌ Failed to write {}: {}", output_file, e);
                process::exit(1);
            }
            println!("✅ HTML written to {}", output_file);
        }
        None => {
            println!("{}", html_output);
        }
    }
}
