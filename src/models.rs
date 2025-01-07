use serde::{Serialize,Deserialize};

/// Selectable
///
pub trait Selectable {
  fn selected(&self) -> &str;
  fn toggle_status(&mut self);
}

fn default_selected() -> bool {
  false
}

/// Extension
/// Represents a Top Level Domain (.com, .net, .org...)
#[derive(Serialize, Deserialize, Debug)]
pub struct Extension {
  pub(crate) tld: String,
  pub(crate) name: String,
  #[serde(default = "default_selected")]
  pub(crate) selected: bool
}

impl Extension {
  fn tld(&self) -> &str {
    &self.tld
  }

  fn name(&self) -> &str {
    &self.name
  }
}

impl Selectable for Extension {
  fn selected(&self) -> &str {
    if self.selected {
      "Selected"
    } else {
      "Not selected"
    }
  }

  fn toggle_status(&mut self) {
    self.selected = !self.selected;
  }
}

/// Domain
/// Represents a Domain (example.com, example.net...)
#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
  pub(crate) domain: String,
  pub(crate) tld: String,
  pub(crate) status: String,
  #[serde(default = "default_selected")]
  pub(crate) selected: bool
}

impl Domain {
  fn domain(&self) -> &str {
    &self.domain
  }

  fn tld(&self) -> &str {
    &self.tld
  }

  pub(crate) fn available(&self) -> &str {
    if self.status == "True" {
      "Available"
    } else {
      "Not available"
    }
  }

  pub(crate) fn domain_name(&self) -> String {
    format!("{}.{}", self.domain, self.tld).to_string()
  }
}

impl Selectable for Domain {
  fn selected(&self) -> &str {
    if self.selected {
      "Wishlisted"
    } else {
      "Not wishlisted"
    }
  }

  fn toggle_status(&mut self) {
    self.selected = !self.selected;
  }
}

impl Clone for Domain {
  fn clone(&self) -> Self {
    Domain {
      domain: String::from(&self.domain),
      tld: String::from(&self.tld),
      status: String::from(&self.status),
      selected: self.selected.clone()
    }
  }
}
