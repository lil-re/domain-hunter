mod models;
mod default_extensions;
mod commands;
mod files;
mod tables;

use commands::{extensions_command, search_command, wishlist_command};
use files::wishlist_file;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Search domain names
    #[arg(short, long, default_value = "")]
    search: String,

    /// Show and manage domain extensions
    #[arg(short, long, default_value_t = false)]
    extensions: bool,

    /// Show and manage favorite domains
    #[arg(short, long, default_value_t = false)]
    wishlist: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.search.len() > 0 {
        search_command::search_domain_names(args.search).await;
    } else if args.extensions {
        extensions_command::handle_extensions();
    } else if args.wishlist {
        wishlist_command::handle_wishlist();
    }
}
