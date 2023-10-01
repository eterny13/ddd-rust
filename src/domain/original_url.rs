use super::url_string::UrlString;
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash)]
pub struct OriginalUrl {
  url_string: UrlString
}

impl OriginalUrl {
  pub fn new (url_string: UrlString) -> Self {
    Self { url_string }
  }
  pub fn get_url_string(self) -> UrlString {
    self.url_string
  }
}