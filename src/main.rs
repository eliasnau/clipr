use std::io::{self, Read};
use std::fs;
use clipboard::copy_to_clipboard;
use clap::{Parser, Subcommand};
use colored::*;

mod clipboard;

#[derive(Parser)]
#[command(
    name = "clipr",
    about = "A simple clipboard utility for command line",
    version,
    author
)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    append: bool,

    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    Clear,
    History,
    Show,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(command) = args.command {
        match command {
            Commands::Clear => {
                copy_to_clipboard("", false)?;
                eprintln!("{}", "Clipboard cleared.".green());
                return Ok(());
            }
            Commands::History => {
                println!("{}", "Clipboard history feature coming soon!".blue());
                return Ok(());
            }
            Commands::Show => {
                let content = clipboard::get_clipboard_content()?;
                if content.is_empty() {
                    println!("{}", "Clipboard is empty".yellow());
                } else {
                    println!("{}", "Current clipboard content:".blue());
                    println!("{}", content);
                }
                return Ok(());
            }
        }
    }

    if let Some(path) = args.file {
        let content = fs::read_to_string(&path)?;
        copy_to_clipboard(&content, args.append)?;
        if args.verbose {
            eprintln!("Copied file '{}'", path.bold());
        } else {
            eprintln!("{}", "File contents copied to clipboard.".green());
        }
        return Ok(());
    }

    if atty::is(atty::Stream::Stdin) {
        println!("{} v{}", "clipr".bold(), env!("CARGO_PKG_VERSION"));
        println!("Run '{}'", "clipr --help".blue());
        return Ok(());
    }

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    if buffer.trim().is_empty() {
        eprintln!("{}", "No input received. Try piping a command or specifying a file.".red());
        std::process::exit(1);
    }

    copy_to_clipboard(&buffer, args.append)?;
    if args.verbose {
        eprintln!("Input copied to clipboard ({} characters)", buffer.len().to_string().yellow());
    } else {
        eprintln!("{}", "Output copied to clipboard.".green());
    }
    Ok(())
}