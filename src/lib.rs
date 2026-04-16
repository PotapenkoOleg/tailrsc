use clap::Parser;
use std::path::PathBuf;

pub mod offset;

use offset::ParsedOffset;

/// tailrsc - a Rust version of the BSD tail command
#[derive(Debug, Parser)]
#[command(name = "tailrsc", version, about = "Display the last part of a file")]
pub struct Args {
    /// The location is number lines
    #[arg(
        short = 'n',
        long = "lines",
        default_value = "10",
        allow_hyphen_values = true,
        value_parser = parse_offset,
    )]
    pub lines: ParsedOffset,

    /// The location is number bytes
    #[arg(
        short = 'c',
        long = "bytes",
        allow_hyphen_values = true,
        value_parser = parse_offset,
    )]
    pub bytes: Option<ParsedOffset>,

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

/// Parses an offset string for `-n` or `-c` arguments.
///
/// Serves as a clap `value_parser`, producing user-facing error messages
/// that match BSD tail format: `illegal offset -- <value>`.
fn parse_offset(s: &str) -> Result<ParsedOffset, String> {
    s.parse::<ParsedOffset>().map_err(|e| e.to_string())
}

/// Parses command-line arguments into an [`Args`] struct.
pub fn get_args() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests;
