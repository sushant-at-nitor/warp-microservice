#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
  Str,
  I32,
  I64,
  Bool,
  F64,
  DateTime,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
  pub name: String,
  pub ty: FieldType,
  pub unique: bool,
  pub nullable: bool,
}

pub fn parse_field(s: &str) -> Result<FieldDef, String> {
  let parts: Vec<&str> = s.split(':').collect();
  if parts.len() < 2 {
    return Err(format!("'{}': expected name:Type[:modifier]", s));
  }
  let name = parts[0];
  if name == "id" {
    return Err("'id' is reserved and auto-generated as PK".to_string());
  }
  let ty = match parts[1] {
    "String"   => FieldType::Str,
    "i32"      => FieldType::I32,
    "i64"      => FieldType::I64,
    "bool"     => FieldType::Bool,
    "f64"      => FieldType::F64,
    "DateTime" => FieldType::DateTime,
    other => {
      return Err(format!(
        "unknown type '{}' — valid: String i32 i64 bool f64 DateTime",
        other
      ))
    }
  };
  let unique   = parts.iter().any(|&p| p == "unique");
  let nullable = !parts.iter().any(|&p| p == "required");
  Ok(FieldDef { name: name.to_string(), ty, unique, nullable })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_basic_string_field() {
    let f = parse_field("title:String").unwrap();
    assert_eq!(f.name, "title");
    assert_eq!(f.ty, FieldType::Str);
    assert!(!f.unique);
    assert!(f.nullable);
  }

  #[test]
  fn parses_unique_modifier() {
    let f = parse_field("email:String:unique").unwrap();
    assert!(f.unique);
    assert!(f.nullable);
  }

  #[test]
  fn parses_required_modifier() {
    let f = parse_field("name:String:required").unwrap();
    assert!(!f.unique);
    assert!(!f.nullable);
  }

  #[test]
  fn rejects_id_field() {
    assert!(parse_field("id:i32").is_err());
  }

  #[test]
  fn rejects_unknown_type() {
    assert!(parse_field("foo:Uuid").is_err());
  }

  #[test]
  fn rejects_missing_type() {
    assert!(parse_field("justname").is_err());
  }

  #[test]
  fn parses_all_six_types() {
    for ty in &["String", "i32", "i64", "bool", "f64", "DateTime"] {
      assert!(
        parse_field(&format!("field:{}", ty)).is_ok(),
        "failed for {}",
        ty
      );
    }
  }
}
