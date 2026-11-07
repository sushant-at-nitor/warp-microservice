pub fn to_pascal_case(s: &str) -> String {
  s.split(['_', '-', ' '])
    .filter(|p| !p.is_empty())
    .map(|p| {
      let mut c = p.chars();
      match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
      }
    })
    .collect()
}

pub fn to_snake_case(s: &str) -> String {
  let mut result = String::new();
  for (i, ch) in s.char_indices() {
    if ch.is_uppercase() && i > 0 {
      result.push('_');
    }
    result.push(ch.to_lowercase().next().unwrap());
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pascal_case_from_snake() {
    assert_eq!(to_pascal_case("book_loan"), "BookLoan");
  }

  #[test]
  fn pascal_case_single_word() {
    assert_eq!(to_pascal_case("member"), "Member");
  }

  #[test]
  fn snake_case_from_pascal() {
    assert_eq!(to_snake_case("BookLoan"), "book_loan");
  }

  #[test]
  fn snake_case_single_word() {
    assert_eq!(to_snake_case("Member"), "member");
  }
}
