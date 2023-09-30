use super::url_string::UrlString;

#[derive(Eq, PartialEq)]
pub struct OriginalUrl {
  url_string: UrlString
}

impl OriginalUrl {
  pub fn new (url_string: UrlString) -> OriginalUrl {
    OriginalUrl { url_string }
  }
  pub fn get_url_string(self) -> UrlString {
    self.url_string
  }
}