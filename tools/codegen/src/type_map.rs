use crate::field_def::FieldType;

impl FieldType {
  pub fn rust_type(&self, nullable: bool) -> String {
    let base = match self {
      FieldType::Str      => "String",
      FieldType::I32      => "i32",
      FieldType::I64      => "i64",
      FieldType::Bool     => "bool",
      FieldType::F64      => "f64",
      FieldType::DateTime => "DateTimeUtc",
    };
    if nullable {
      format!("Option<{}>", base)
    } else {
      base.to_string()
    }
  }

  pub fn seaorm_col_method(&self) -> &str {
    match self {
      FieldType::Str      => "string()",
      FieldType::I32      => "integer()",
      FieldType::I64      => "big_integer()",
      FieldType::Bool     => "boolean()",
      FieldType::F64      => "double()",
      FieldType::DateTime => "timestamp_with_time_zone()",
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::field_def::FieldType;

  #[test]
  fn string_nullable_wraps_in_option() {
    assert_eq!(FieldType::Str.rust_type(true), "Option<String>");
  }

  #[test]
  fn i32_required_is_bare() {
    assert_eq!(FieldType::I32.rust_type(false), "i32");
  }

  #[test]
  fn datetime_nullable() {
    assert_eq!(FieldType::DateTime.rust_type(true), "Option<DateTimeUtc>");
  }

  #[test]
  fn seaorm_string_method() {
    assert_eq!(FieldType::Str.seaorm_col_method(), "string()");
  }

  #[test]
  fn seaorm_integer_method() {
    assert_eq!(FieldType::I32.seaorm_col_method(), "integer()");
  }

  #[test]
  fn seaorm_datetime_method() {
    assert_eq!(FieldType::DateTime.seaorm_col_method(), "timestamp_with_time_zone()");
  }
}
