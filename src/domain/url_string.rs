#[derive(Eq, PartialEq)]
pub struct UrlString {
  value: String,
}

impl UrlString {
  pub fn new(value: String) -> UrlString {
    UrlString { value }
  }
  pub fn get_value(self) -> String {
    self.value
  }
}
