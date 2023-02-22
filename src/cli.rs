use clap::Parser;

/// Template cli tool
#[derive(Parser)]
pub struct Args {
    /*
    // Message string
    #[arg(short, long, default_value = "default msg")]
    message: String,
    */

    /// Log to file
    #[arg(long)]
    pub log_file: Option<String>,

    /// Verbose mode (-v, -vv, etc.)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
