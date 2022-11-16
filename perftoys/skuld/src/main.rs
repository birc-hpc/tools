use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::io::Read;
use terminal_size::{terminal_size, Width};

// Working with terminal...
fn get_term_width() -> u32 {
    if let Some((Width(term_width), ..)) = terminal_size() {
        term_width as u32
    } else {
        80
    }
}

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
struct Cli {
    /// The number of times to process the input file
    #[arg(short, default_value_t = 1000)]
    n: u32,

    /// Which technique do you want for processing the data?
    #[command(subcommand)]
    action: Action,
}

/// Action to run. All actions solve the same problem,
/// computing a historgram over the bytes used in the
/// input file, but they use different resources doing so.
#[derive(Subcommand)]
enum Action {
    /// Low memory, high I/O solution.
    ///
    /// Scans the file in small blocks, avoiding loading it all
    /// into memory. This is costly in I/O but saves on RAM.
    Brokkr {
        /// The file to process. This cannot be a pipe but must be
        /// a proper file.
        path: std::path::PathBuf,
    },

    /// High memory, low I/O
    ///
    /// Loads the entire file into memory so it only has to read it
    /// from file once. It then processes the data without further
    /// disk access. This is cheap in I/O but expensive in memory.
    Eitri {
        /// The file to process. This cannot be a pipe but must be
        /// a proper file.
        path: std::path::PathBuf,
    },

    /// Higher memory, low I/O
    ///
    /// Loads the entire file into memory so it only has to read it
    /// from file once. It then processes the data without further
    /// disk access. This is cheap in I/O but expensive in memory.
    Sindri {
        /// The file to process. This cannot be a pipe but must be
        /// a proper file.
        path: std::path::PathBuf,
    },
}

fn scan_command(path: &std::path::PathBuf, n: u32, counts: &mut [u32; 256]) -> Result<()> {
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];

    for _ in 0..n {
        let mut file = std::fs::File::open(path)
            .with_context(|| format!("could not open file `{}`", path.display()))?;

        loop {
            let read_count = file.read(&mut buffer)?;
            skuld::count_bytes(&buffer[..read_count], counts);
            if read_count != BUFFER_LEN {
                break;
            }
        }
    }
    Ok(())
}

fn load_command(path: &std::path::PathBuf, n: u32, counts: &mut [u32; 256]) -> Result<()> {
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    for _ in 0..n {
        skuld::count_bytes(content.as_bytes(), counts);
    }
    Ok(())
}

fn crazy_load_command(path: &std::path::PathBuf, n: u32, counts: &mut [u32; 256]) -> Result<()> {
    let mut strings: Vec<String> = vec![];
    for _ in 0..n {
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("could not read file `{}`", path.display()))?;
        strings.push(content); // just to waste some memory
        skuld::count_bytes(strings.last().unwrap().as_bytes(), counts);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut counts = [0u32; 256];

    match args.action {
        // Dispatch to sub-command
        Action::Brokkr { path } => scan_command(&path, args.n, &mut counts),
        Action::Eitri { path } => load_command(&path, args.n, &mut counts),
        Action::Sindri { path } => crazy_load_command(&path, args.n, &mut counts),
    }?;

    let margin = 5; // 'nnn: '
    let term_width = get_term_width();
    skuld::fit_counts_to_termwidth(&mut counts, term_width - margin);
    skuld::print_counts(&counts);

    Ok(())
}
