# Domain Hunter

A command line interface (CLI) tool to search for available domain names and save your favorites to a wishlist.

## Usage

Usage :

```bash
domainhunter [options]
```

Options :

```bash
-s, --search <DOMAIN_NAME>  Search for available domain names
-e, --extensions            Show and manage domain name extensions
-w, --wishlist              Show and manage your wishlist of domain names
-h, --help                  Print help message
-V, --version               Print version information
```

## Search

To search for domain names, use the `--search <DOMAIN_NAME>` option. You don't need to specify the domain extension 
(e.g., `.com`, `.net`). Domain Hunter will automatically check availability for multiple common extensions (default:
`.com`, `.net`, `.org`).

Example:

```bash
domainhunter --search example
```

This will check for domain names like `example.com`, `example.net`, and `example.org`.

## Extensions

By default, Domain Hunter checks for `.com`, `.net`, and `.org` extensions. To customize the domain extensions, use the 
`--extensions` option. This will create or update the `extensions.json` file with your selected extensions.  
This command will show the available extensions and allow you to manage your preferences.

## Wishlist

You can add available domain names to your wishlist during the search process. The wishlist is stored in a 
`wishlist.json` file.  
To view your wishlist, use the `--wishlist` option. From here, you can also remove domain names from your wishlist.
