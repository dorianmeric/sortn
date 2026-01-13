use clap::Parser;
use std::io::{self, BufRead, Write};

/// A utility to perform natural sorting on input lines.
#[derive(Parser, Debug)]
#[command(
    name = "sortn",
    version,
    about = "Sorts lines naturally (e.g., '2' before '10')",
    long_about = None
)]
struct Args {
    /// Sort in reverse order
    #[arg(short, long)]
    reverse: bool,

    /// Case-insensitive sorting
    #[arg(short, long)]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();

    let stdin = io::stdin();
    let stdout = io::stdout();
    
    // 1. Collect all lines into memory as Strings
    // Natural sorting requires string comparison. We collect them into a Vec
    // so we can perform the sort operation on the whole collection.
    let mut lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|l| l.expect("Failed to read line from stdin"))
        .collect();

    // 2. Perform natural sort
    // We use the natord crate for "natural" alphanumeric ordering.
    lines.sort_by(|a, b| {
        let cmp = if args.ignore_case {
            // Compare lowercase versions for case-insensitivity
            natord::compare(&a.to_lowercase(), &b.to_lowercase())
        } else {
            natord::compare(a, b)
        };

        if args.reverse {
            cmp.reverse()
        } else {
            cmp
        }
    });

    // 3. Output the sorted lines
    let mut out = stdout.lock();
    for line in lines {
        if let Err(e) = writeln!(out, "{}", line) {
            // Handle broken pipes (e.g., when piping to `head`) gracefully
            if e.kind() == io::ErrorKind::BrokenPipe {
                break;
            }
            panic!("Failed to write output: {}", e);
        }
    }
}
