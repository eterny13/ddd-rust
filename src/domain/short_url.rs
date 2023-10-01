use super::hashed_url_string::HashedUrlString;

#[derive(Eq, PartialEq)]
pub struct ShortUrl {
  hashed_url_string: HashedUrlString 
}

impl ShortUrl {
  pub fn new(hashed_url_string: HashedUrlString) -> Self {
    Self { hashed_url_string }
  }

  pub fn get_hashed_url_string(self) -> HashedUrlString {
    self.hashed_url_string
  }
}
