mod models;
mod commands;
mod files;
mod tables;
mod database;

use commands::{extensions_command, search_command, wishlist_command, init_command};
use clap::Parser;
use database::{connection};

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

    /// Init app and database
    #[arg(short, long, default_value_t = false)]
    init: bool,
}

#[tokio::main]
async fn main() {
    if let Err(e) = connection::establish_connection() {
        eprintln!("Error establishing connection: {}", e);
        return;
    }

    let args = Args::parse();

    if args.search.len() > 0 {
        search_command::search_domain_names(args.search).await;
    } else if args.extensions {
        extensions_command::handle_extensions();
    } else if args.wishlist {
        wishlist_command::handle_wishlist();
    } else if args.init {
        init_command::handle_init();
    }
}
