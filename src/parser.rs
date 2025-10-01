use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// files to be inputted
    files: Option<Vec<String>>,
}
