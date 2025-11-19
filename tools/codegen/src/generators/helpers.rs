use inflector::cases::camelcase::to_camel_case;
use inflector::cases::pascalcase::to_pascal_case;
use inflector::cases::snakecase::to_snake_case;
use inflector::cases::screamingsnakecase::to_screaming_snake_case;
use inflector::string::pluralize::to_plural;

/// Converts a PascalCase entity name to snake_case (e.g., "Book" -> "book").
pub fn to_snake_name(entity_name: &str) -> String {
    to_snake_case(entity_name)
}

/// Converts a PascalCase entity name to snake_case plural (e.g., "Book" -> "books").
pub fn to_plural_snake_name(entity_name: &str) -> String {
    to_plural(&to_snake_name(entity_name))
}

/// Converts a PascalCase entity name to camelCase (e.g., "BookPlan" -> "bookPlan").
pub fn to_camel_name(entity_name: &str) -> String {
    to_camel_case(entity_name)
}

/// Converts a PascalCase entity name to PascalCase (idempotent, e.g., "BookPlan" -> "BookPlan").
pub fn to_pascal_name(entity_name: &str) -> String {
    to_pascal_case(entity_name)
}

/// Converts a PascalCase entity name to SCREAMING_SNAKE_CASE (e.g., "BookPlan" -> "BOOK_PLAN").
pub fn to_screaming_snake_name(entity_name: &str) -> String {
    to_screaming_snake_case(entity_name)
}

/// Converts a PascalCase entity name to camelCase plural (e.g., "BookPlan" -> "bookPlans").
pub fn to_plural_camel_name(entity_name: &str) -> String {
    to_plural(&to_camel_name(entity_name))
}

/// Converts a PascalCase entity name to PascalCase plural (e.g., "BookPlan" -> "BookPlans").
pub fn to_plural_pascal_name(entity_name: &str) -> String {
    to_plural(&to_pascal_name(entity_name))
}

/// Converts a PascalCase entity name to SCREAMING_SNAKE_CASE plural (e.g., "BookPlan" -> "BOOK_PLANS").
pub fn to_plural_screaming_snake_name(entity_name: &str) -> String {
    to_plural(&to_screaming_snake_name(entity_name))
}
