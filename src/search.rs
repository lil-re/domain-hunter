use reqwest::Url;
use regex::Regex;
use serde::{Deserialize};
use crate::search_ui::display_search_result;

#[derive(Deserialize, Debug)]
struct DomainStatus {
  auctionsite: Option<String>,
  status: String,
  tld: String,
  domain: String,
  origidomain: Option<String>
}

pub async fn search_domain_names(domain: String) {
  println!("Search domain : {}", domain);
  let extensions: String = get_extensions();
  let url: Url = get_url(domain, extensions);
  let response: String = get_domains(url).await;
  let data: Vec<DomainStatus> = parse_response(response);
  display_search_result().expect("TODO: panic message");

  // TODO => Draw table with Ratatui to display data
}

pub fn get_extensions() -> String {
  let default_extensions: Vec<&str> = vec!["com", "net", "org", "co", "io", "ai"];
  let formatted_extensions: String = default_extensions
      .iter()
      .map(|&e| format!("\"{}\"", e))
      .collect::<Vec<_>>()
      .join(",");

  format!("[{}]", formatted_extensions)
}

pub fn get_url(domain: String, extensions: String) -> Url {
  let url = "https://domaintyper.com/API/DomainCheckAsync";
  let params = [
    ("domain", domain),
    ("tlds", extensions)
  ];

  Url::parse_with_params(url, &params).unwrap_or_else(|error| { panic!("{}", error) })
}

pub async fn get_domains(url: Url) -> String {
  let response = match reqwest::get(url).await {
    Ok(response) => response,
    Err(error) => { panic!("{}", error) }
  };

  response.text().await.unwrap_or_else(|error| { panic!("{}", error) })
}

pub fn parse_response(response: String) -> Vec<DomainStatus> {
  let re = Regex::new(r"}\{").unwrap();
  let formatted = re.replace_all(&*response, "},{");
  let json_data = format!("[{}]", formatted);
  serde_json::from_str(&*json_data).unwrap()
}
