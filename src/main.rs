mod search;
mod extensions;
mod wishlist;
mod search_ui;
mod extensions_ui;
mod tables;

use clap::Parser;
use crate::extensions::handle_extensions;
use crate::wishlist::handle_wishlist;
use crate::search::search_domain_names;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Search domain names
    #[arg(short, long, default_value = "")]
    search: String,

    /// Manage domain extensions
    #[arg(short, long, default_value_t = false)]
    extensions: bool,

    /// Manage favorite domains
    #[arg(short, long, default_value_t = false)]
    wishlist: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.search.len() > 0 {
        search_domain_names(args.search).await;
    } else if args.extensions {
        handle_extensions();
    } else if args.wishlist {
        handle_wishlist();
    }
}
