use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

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

fn load_command(path: &std::path::PathBuf, n: u32) -> Result<()> {
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    println!("Here I should scan the file {} times", n);
    println!("file content: {}", content);
    Ok(())
}

fn scan_command(path: &std::path::PathBuf, n: u32) -> Result<()> {
    std::fs::File::open(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    println!("I plan to scan the file here! I promise!");
    println!("Here I should scan the file {} times", n);
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.action {
        // Dispatch to sub-command
        Action::Load { path } => load_command(&path, args.n),
        Action::Scan { path } => scan_command(&path, args.n),
    }
}
