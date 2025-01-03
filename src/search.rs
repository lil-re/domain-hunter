use reqwest::Url;
use regex::Regex;
use crate::search_ui::{display_search_result, Domain};

pub async fn search_domain_names(domain: String) {
  let extensions: String = get_extensions();
  let url: Url = get_url(domain, extensions);
  let data: String = search_domains(url).await;
  let domains: Vec<Domain> = parse_data(data);
  display_search_result(domains).expect("An error occurred while displaying results");
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

  match Url::parse_with_params(url, &params) {
    Ok(response) => response,
    Err(error) => { panic!("{}", error) }
  }
}

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

pub fn parse_data(raw_data: String) -> Vec<Domain> {
  let re = Regex::new(r"}\{").unwrap();
  let formatted_data = re.replace_all(&*raw_data, "},{");
  let json_data = format!("[{}]", formatted_data);
  let mut parsed_data: Vec<Domain> = serde_json::from_str(&*json_data).unwrap();
  parsed_data.sort_by(|a, b| a.tld.cmp(&b.tld));
  parsed_data
}
