use clap::Parser;
use std::fs;
use std::io::{self, Read};

mod font;
mod markdown;
mod image;
mod terminal;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input markdown file
    #[arg(value_name = "FILE")]
    file: Option<String>,
    
    /// Read from stdin
    #[arg(short = 's', long = "stdin")]
    stdin: bool,
    
    /// Render width in pixels (text is wrapped to fit)
    #[arg(short = 'w', long = "width", default_value_t = 800)]
    width: usize,
    
    /// Color theme
    #[arg(long = "theme", default_value = "light")]
    theme: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let markdown_content = if args.stdin {
        let mut stdin = io::stdin();
        let mut content = String::new();
        stdin.read_to_string(&mut content)?;
        content
    } else if let Some(file) = args.file {
        fs::read_to_string(&file)?
    } else {
        eprintln!("No input specified");
        std::process::exit(1);
    };
    
    // Process markdown and render to terminal
    let rendered = markdown::process_markdown(&markdown_content, args.width);
    
    println!("{}", rendered);
    
    Ok(())
}