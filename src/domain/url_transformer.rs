use super::original_url::OriginalUrl;

pub fn transform_short_url(original_url: OriginalUrl) -> String {
  original_url.get_url_string().get_value()
}

#[cfg(test)]
mod tests {
  use crate::domain::url_string::UrlString;
  use crate::domain::original_url::OriginalUrl;
  use super::*;

  #[test]
  fn url_transformer_spec() {
    let url = OriginalUrl::new(UrlString::new("example.com".to_string()));
    assert_eq!(transform_short_url(url), "example.com");
  }
} 
