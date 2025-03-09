use clap::Parser;

use crate::commands::Commands;

#[derive(Parser, Debug)]
#[command(name = "nyxpng")]
#[command( about="A CLI tool to encode, decode, remove ", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
