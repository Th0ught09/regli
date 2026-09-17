use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// files to be inputted
    files: Vec<String>,
    /// Directory to search
    #[arg(short, long, default_value_t = String::from("."))]
    dir: String,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

pub fn get_files(cli: Cli) -> Vec<String> {
    cli.files
}

pub fn get_dir(cli: Cli) -> String {
    cli.dir
}
