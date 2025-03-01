use reqwest::Url;
use regex::Regex;
use crate::database::extensions_api::find_selected_extensions;
use crate::database::wishlist_api::find_wishlist;
use crate::tables::domains_table::display_domains;
use crate::models::{Domain, Extension};

pub async fn search_domain_names(domain: String) {
  let extensions: String = get_selected_extensions();
  let url: Url = get_url(domain, extensions);
  let data: String = search_domains(url).await;
  let mut domains: Vec<Domain> = parse_data(data);
  let wishlist : Vec<Domain> = find_wishlist();
  set_wishlisted_domain(&mut domains, wishlist);
  display_domains(domains).expect("An error occurred while displaying results");
}

/// Get the extensions selected by the user
pub fn get_selected_extensions() -> String {
  let selected_extensions: Vec<Extension> = find_selected_extensions();
  let selected_extensions_tlds: String = selected_extensions
      .iter()
      .map(|e| format!("\"{}\"", e.tld))
      .collect::<Vec<_>>()
      .join(",");

  format!("[{}]", selected_extensions_tlds)
}

/// Generate the URL to fetch domain names
pub fn get_url(domain: String, extensions: String) -> Url {
  let url = "https://domaintyper.com/API/DomainCheckAsync";
  let params = [
    ("domain", domain),
    ("tlds", extensions)
  ];

  match Url::parse_with_params(url, &params) {
    Ok(response) => response,
    Err(error) => { panic!("{}", error) }
  }
}

/// Search for domain names
pub async fn search_domains(url: Url) -> String {
  let response = match reqwest::get(url).await {
    Ok(response) => response,
    Err(error) => { panic!("{}", error) }
  };

  match response.text().await {
    Ok(response) => response,
    Err(error) => { panic!("{}", error) }
  }
}

/// Parse search result and transform data into a vector of Domain
pub fn parse_data(raw_data: String) -> Vec<Domain> {
  let re = Regex::new(r"}\{").unwrap();
  let formatted_data = re.replace_all(&*raw_data, "},{");
  let json_data = format!("[{}]", formatted_data);
  let mut parsed_data: Vec<Domain> = serde_json::from_str(&*json_data).unwrap();
  parsed_data.sort_by(|a, b| a.tld.cmp(&b.tld));
  parsed_data
}

/// Retrieve the list of wishlisted domains to check if the user has already added domains in the wishlist
pub fn set_wishlisted_domain(result: &mut Vec<Domain>, wishlist: Vec<Domain>) {
  let wishlisted_domain_names: Vec<String> = wishlist.iter().map(|d| d.domain_name()).collect();

  for domain in result {
    if wishlisted_domain_names.contains(&domain.domain_name().to_string()) {
      domain.selected = true
    }
  }
}
