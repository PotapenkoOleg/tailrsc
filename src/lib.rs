use clap::Parser;
use std::path::PathBuf;

/// tailrsc - a Rust version of the BSD tail command
#[derive(Debug, Parser)]
#[command(name = "tailrsc", version, about = "Rust version of the BSD tail command")]
pub struct Args {
    /// The location is number lines
    #[arg(
        short = 'n',
        long = "lines",
        default_value = "10",
        allow_hyphen_values = true
    )]
    pub lines: String,

    /// The location is number bytes
    #[arg(short = 'c', long = "bytes", allow_hyphen_values = true)]
    pub bytes: Option<String>,

    /// Do not stop when end of file is reached; wait for additional data
    #[arg(short = 'f', long = "follow")]
    pub follow: bool,

    /// Suppresses printing of headers when multiple files are examined
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,

    /// Prepend each file with a header
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Input files
    #[arg(value_name = "FILE")]
    pub files: Vec<PathBuf>,
}

pub fn get_args() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests;
