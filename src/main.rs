use clap::Parser;

/// Replace pattern in file name
#[derive(Parser)]
struct Cli {
    /// pattern to look for
    pattern: String,
    /// path to file
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
}
