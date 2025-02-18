use std::io::{self, Read};
use std::fs;
use clipboard::copy_to_clipboard;
use clap::{Parser, Subcommand};

mod clipboard;

#[derive(Parser)]
#[command(
    name = "clipr",
    about = "A simple clipboard utility for command line",
    version,
    author
)]
struct Args {
    /// Subcommand to execute
    #[command(subcommand)]
    command: Option<Commands>,

    /// File to read from
    #[arg(short, long)]
    file: Option<String>,

    /// Append to existing clipboard content instead of replacing
    #[arg(short, long)]
    append: bool,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Clear clipboard content
    Clear,
    /// Show clipboard history (coming soon)
    History,
    /// Show clipboard content
    Show,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Handle subcommands
    if let Some(command) = args.command {
        match command {
            Commands::Clear => {
                copy_to_clipboard("", false)?;
                eprintln!("Clipboard cleared.");
                return Ok(());
            }
            Commands::History => {
                println!("Clipboard history feature coming soon!");
                return Ok(());
            }
            Commands::Show => {
                let content = clipboard::get_clipboard_content()?;
                println!("{}", content);
                return Ok(());
            }
        }
    }

    // Handle file input
    if let Some(path) = args.file {
        let content = fs::read_to_string(&path)?;
        copy_to_clipboard(&content, args.append)?;
        if args.verbose {
            eprintln!("File '{}' contents copied to clipboard.", path);
        } else {
            eprintln!("File contents copied to clipboard.");
        }
        return Ok(());
    }

    // Handle pipe input
    if atty::is(atty::Stream::Stdin) {
        println!("clipr v{}", env!("CARGO_PKG_VERSION"));
        println!("Run 'clipr --help' for usage information");
        return Ok(());
    }

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    if buffer.trim().is_empty() {
        eprintln!("No input received. Try piping a command or specifying a file.");
        std::process::exit(1);
    }

    copy_to_clipboard(&buffer, args.append)?;
    if args.verbose {
        eprintln!("Input copied to clipboard ({} characters).", buffer.len());
    } else {
        eprintln!("Output copied to clipboard.");
    }
    Ok(())
}