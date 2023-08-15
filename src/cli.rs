use std::path::PathBuf;
use std::str::FromStr;
use clap::Parser;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Activate debug mode
    // short and long flags, (-d, --debug) derived from the field name
    #[arg(short, long)]
    debug: bool,

    /// The command to execute
    #[arg(short, long)]
    pub command: String,
    
    /// The user ID to use in the container
    #[arg(short, long)]
    pub uid: u32,

    /// The directory to be mounted as root for the container
    #[arg(short=char::from_str("m").unwrap(), long="mount")]
    pub mount_dir: PathBuf,

}

pub fn parse_args() -> Args {
    let args = Args::parse();
    args
}