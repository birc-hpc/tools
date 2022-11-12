use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use perflib::{count_bytes, fit_counts_to_termwidth, print_counts};
use std::io::Read;

/// Skuld is one of the three Norns in Norse mythology.
/// The word likely means debt, and this tool will put
/// you in resource debt in various ways.
///
/// Well, not directly in debt, but it will let you run
/// different computations that will use different
/// resources for solving the same problem, and through
/// that let you explore how many resources you would
/// need to use it if you submit it to the cluster queue.
///
/// The job that Skuld actually does in this tool is to
/// read data from a text file and count how often each
/// character appears in the input. It then outputs
/// a summary of what it saw.
///
/// Different sub-commands will solve this job in different
/// ways, using different resources in the process.
#[derive(Parser)]
#[command(author, version, about, long_about)]
#[command(propagate_version = true)]
struct Cli {
    /// The number of times to process the input file
    #[arg(short, default_value_t = 1)]
    n: u32,

    /// Which technique do you want for processing the data?
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Load the input file into memory and process it there
    Load {
        /// The file to process. This cannot be a pipe but must be
        /// a proper file.
        path: std::path::PathBuf,
    },

    /// Scan the file without loading all of it in at once
    Scan {
        /// The file to process. This cannot be a pipe but must be
        /// a proper file.
        path: std::path::PathBuf,
    },
}

fn load_command(path: &std::path::PathBuf, n: u32, counts: &mut [u32; 256]) -> Result<()> {
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    for _ in 0..n {
        count_bytes(content.as_bytes(), counts);
    }
    Ok(())
}

fn scan_command(path: &std::path::PathBuf, n: u32, counts: &mut [u32; 256]) -> Result<()> {
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];

    for _ in 0..n {
        let mut file = std::fs::File::open(path)
            .with_context(|| format!("could not open file `{}`", path.display()))?;

        loop {
            let read_count = file.read(&mut buffer)?;
            count_bytes(&buffer, counts);

            if read_count != BUFFER_LEN {
                break;
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut counts = [0u32; 256];

    match args.action {
        // Dispatch to sub-command
        Action::Load { path } => load_command(&path, args.n, &mut counts),
        Action::Scan { path } => scan_command(&path, args.n, &mut counts),
    }?;

    fit_counts_to_termwidth(&mut counts);
    print_counts(&counts);

    Ok(())
}
