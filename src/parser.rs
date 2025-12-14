use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// files to be inputted
    files: Vec<String>,
}

pub fn parse_args() -> Vec<String> {
    let cli = Cli::parse();
    let files: Vec<String> = cli.files;
    files
}
