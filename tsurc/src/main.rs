use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tsurc")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path for the File to be executed
    #[arg(required_unless_present("ast"))]
    file: Option<PathBuf>,
    /// path for the AST to be executed
    #[arg(short, long)]
    ast: Option<PathBuf>,
    /// Verbose,
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{:?}", args);
    Ok(())
}
