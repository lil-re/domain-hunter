# Domain Hunter

A command line interface (CLI) tool to search for available domain names and save your favorites to a wishlist.

## Usage

Usage :

```bash
domain-hunter [options]
```

Options :

```
-i, --init <DOMAIN_NAME>    Initializes local database
-s, --search <DOMAIN_NAME>  Search for available domain names
-e, --extensions            Show and manage domain name extensions
-w, --wishlist              Show and manage your wishlist of domain names
-h, --help                  Print help message
-V, --version               Print version information
```

## Initializing 

To be able to use Domain Hunter, you have to initialize its database by running `domain-hunter --init`. This command creates
a SQLite database with 2 tables : 
 - `extension`: Store a list of top level domain such as `.com`, `.net`, `.org`...
 - `wishlist`: Store the user's domain names wishlist.

## Search

To search for domain names, use the `--search <DOMAIN_NAME>` option. You don't need to specify the domain extension 
(e.g., `.com`, `.net`). Domain Hunter will automatically check availability for multiple common extensions (default:
`.com`, `.net`, `.org`).

Example:

```bash
domain-hunter --search example
```

This will check for domain names like `example.com`, `example.net`, and `example.org`.

## Extensions

By default, Domain Hunter checks for `.com`, `.net`, and `.org` extensions. To customize the domain extensions, use the 
`domain-hunter --extensions` command. This command shows a list of extensions and allow you to manage your preferences 
by selecting or unselecting the top level domains used during a domain name search. Your extensions preferences are 
stored in the `extension` table in the local database.

## Wishlist

You can add domain names to your wishlist during the search process. The wishlist is stored the `wishlist` table in the 
local database. To view your wishlist, use the `domain-hunter --wishlist` command. From here, you can also remove domain 
names from your wishlist.
