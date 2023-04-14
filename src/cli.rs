use std::path::PathBuf;

use clap::Parser;

pub fn parse() -> Cli {
    Cli::parse()
}

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Minimum usable field strength in dB(uV)/m.
    pub min_field_strength: f64,

    /// Input csv file.
    #[arg(value_parser = parse_input_path)]
    pub input_file: Option<PathBuf>,

    // #[arg(value_parser = parse_output_path)]
    // pub output_file: Option<PathBuf>,

    /// Disables plotting of every result.
    #[arg(long)]
    pub no_plot: bool,
}

fn parse_input_path(input: &str) -> Result<PathBuf, String> {
    let path: PathBuf = input
        .parse()
        .map_err(|_| format!("`{input}` isn't a valid path."))?;
    if path.is_file() {
        Ok(path)
    } else {
        Err(format!("`{}` is not a file.", path.display()))
    }
}

fn parse_output_path(input: &str) -> Result<PathBuf, String> {
    let path: PathBuf = input
        .parse()
        .map_err(|_| format!("`{input}` isn't a valid path."))?;
    if !path.exists() || path.is_file() {
        Ok(path)
    } else {
        Err(format!("`{}` is not a file.", path.display()))
    }
}
