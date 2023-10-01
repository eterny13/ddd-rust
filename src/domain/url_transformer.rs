use std::{collections::hash_map::DefaultHasher, hash::Hasher};
use std::hash::Hash;

use super::short_url::ShortUrl;
use super::{original_url::OriginalUrl, hashed_url_string::HashedUrlString};

pub fn transform_short_url(original_url: OriginalUrl) -> ShortUrl {
  let hashed_url_string = transform_hashed_url_string(original_url);
  let short_url = ShortUrl::new(hashed_url_string);
  short_url
}

pub fn transform_hashed_url_string(url: OriginalUrl) -> HashedUrlString {
  let mut hasher = DefaultHasher::new();
  url.get_url_string().hash(&mut hasher);
  let hashed_url_string = HashedUrlString::new(hasher.finish().to_string());
  hashed_url_string
}

#[cfg(test)]
mod tests {
  use crate::domain::url_string::UrlString;
  use crate::domain::original_url::OriginalUrl;
  use super::*;

  #[test]
  fn url_transformer_spec() {
    let url = OriginalUrl::new(UrlString::new("example.com".to_string()));
    let short_url = transform_short_url(url);
    assert_eq!(short_url.get_hashed_url_string().value(), "5257664237369877164");
  }
} 
