use serde::{Serialize,Deserialize};

/// Selectable
pub trait Selectable {
  fn is_selected(&self) -> String;
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

impl Selectable for Extension {
  fn is_selected(&self) -> String {
    if self.selected {
      String::from("Selected")
    } else {
      String::from("Not selected")
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
  pub(crate) fn is_available(&self) -> String {
    if self.status == "True" {
      String::from("Available")
    } else {
      String::from("Not available")
    }
  }

  // Complete domain domain name (e.g. "example.com")
  pub(crate) fn domain_name(&self) -> String {
    format!("{}.{}", self.domain, self.tld)
  }
}

impl Selectable for Domain {
  fn is_selected(&self) -> String {
    if self.selected {
      String::from("Wishlisted")
    } else {
      String::from("Not wishlisted")
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
