#[derive(Eq, PartialEq, Clone)]
pub struct HashedUrlString {
  value: String,
}

impl HashedUrlString {
  pub fn new(value: String) -> HashedUrlString {
    HashedUrlString { value }
  }
  
  pub fn value(self) -> String {
    self.value
  } 
}
