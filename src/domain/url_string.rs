#[derive(Eq, PartialEq, Hash)]
pub struct UrlString {
  value: String,
}

impl UrlString {
  pub fn new(value: String) -> Self {
    Self { value }
  }
  pub fn value(self) -> String {
    self.value
  }
}
