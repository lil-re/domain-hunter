mod search_command;
mod extensions_command;
mod wishlist_command;
mod domains_table;
mod extensions_table;
mod tables;
mod models;
mod wishlist_file;
mod extensions_file;
mod default_extensions;

use clap::Parser;
use crate::extensions_command::handle_extensions;
use crate::wishlist_command::handle_wishlist;
use crate::search_command::search_domain_names;

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
