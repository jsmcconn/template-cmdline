use clap::Parser;

/// Template cli tool
#[derive(Parser)]
#[cfg_attr(not(feature = "color"), command(disable_colored_help = true))]
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
