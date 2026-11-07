# Codegen v2 — Field-Aware Scaffold Generator Implementation Plan

> **For agentic workers:** Use `subagent-driven-development` skill (recommended) or
> `executing-plans` to implement task by task. Steps use `- [ ]` checkbox syntax for tracking.

**Goal:** Extend `tools/codegen` to accept `--field name:Type[:modifier]` CLI flags and generate 22 compilable files across all 5 architecture layers.

**Spec:** `docs/specs/2026-11-07-codegen-v2-spec.md`

**Architecture:** A `clap`-driven CLI parses field definitions into `FieldDef` structs; each of 5 generator modules receives `&[FieldDef]` and emits Rust source via `format!`; empty-slice fallback preserves backward compatibility with existing 10 tests.

**Tech Stack:** Rust 2024, clap 4 (derive), chrono 0.4, SeaORM 1.1.17 patterns, Warp 0.4.2.

---

## File Structure Map

| File | Action | Responsibility |
|------|--------|----------------|
| `tools/codegen/Cargo.toml` | Modify | Add clap + chrono deps |
| `tools/codegen/src/field_def.rs` | **Create** | `FieldType` enum, `FieldDef` struct, `parse_field()` |
| `tools/codegen/src/type_map.rs` | **Create** | `FieldType::rust_type()`, `FieldType::seaorm_col_method()` |
| `tools/codegen/src/main.rs` | Modify | clap `Cli` struct, parse fields, dispatch all generators, print checklist |
| `tools/codegen/src/domain_generator.rs` | Modify | Real fields in aggregate, commands, queries |
| `tools/codegen/src/app_generator.rs` | Modify | All 7 handler files, uniform `AppError`, update with field-apply loop |
| `tools/codegen/src/api_generator.rs` | Modify | All 7 endpoints + complete `mod.rs` route chain |
| `tools/codegen/src/migration_generator.rs` | **Create** | `ColumnDef` per field, `up`/`down`, `Iden` enum |
| `tools/codegen/src/infra_generator.rs` | Modify | Entity + mapper + full CRUD repository + service impl |

---

## Tasks

---

### T1 — Add clap + chrono to `tools/codegen/Cargo.toml`

```yaml
task:
  id: 1
  description: "Add clap (derive) and chrono dependencies to the codegen crate"
  skill: null
  parallel: false
  depends_on: []
```

- [ ] Edit `tools/codegen/Cargo.toml`:

```toml
[package]
name = "codegen"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "codegen"
path = "src/main.rs"

[dependencies]
clap = { version = "4", features = ["derive"] }
chrono = "0.4"
```

- [ ] Run: `cargo build -p codegen` — confirm it compiles.
- [ ] Commit: `git add tools/codegen/Cargo.toml && git commit -m "T1: add clap + chrono deps to codegen"`

**Expected output:** `Finished dev profile`
**Verification:** `cargo build -p codegen`

---

### T2 — Implement `tools/codegen/src/field_def.rs`

```yaml
task:
  id: 2
  description: "FieldType enum, FieldDef struct, parse_field() with 7 unit tests"
  skill: test-driven-development
  parallel: false
  depends_on: [1]
```

- [ ] Add `mod field_def;` to `tools/codegen/src/main.rs`.
- [ ] Write the failing tests first in `tools/codegen/src/field_def.rs`:

```rust
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
      assert!(parse_field(&format!("field:{}", ty)).is_ok(), "failed for {}", ty);
    }
  }
}
```

- [ ] Run `cargo test -p codegen field_def` — confirm RED (compile error or no such function).
- [ ] Implement the full module:

```rust
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
    other => return Err(format!("unknown type '{}' — valid: String i32 i64 bool f64 DateTime", other)),
  };
  let unique   = parts.iter().any(|&p| p == "unique");
  let nullable = !parts.iter().any(|&p| p == "required");
  Ok(FieldDef { name: name.to_string(), ty, unique, nullable })
}
```

- [ ] Run `cargo test -p codegen field_def` — confirm GREEN (7 passed).
- [ ] Commit: `git commit -am "T2: implement field_def.rs with 7 tests"`

**Expected output:** `test result: ok. 7 passed`
**Verification:** `cargo test -p codegen field_def`

---

### T3 — Implement `tools/codegen/src/type_map.rs`

```yaml
task:
  id: 3
  description: "FieldType::rust_type() and FieldType::seaorm_col_method() with 6 tests"
  skill: test-driven-development
  parallel: false
  depends_on: [2]
```

- [ ] Add `mod type_map;` to `main.rs`.
- [ ] Write failing tests in `tools/codegen/src/type_map.rs`:

```rust
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
```

- [ ] Run `cargo test -p codegen type_map` — confirm RED.
- [ ] Implement:

```rust
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
    if nullable { format!("Option<{}>", base) } else { base.to_string() }
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
```

- [ ] Run `cargo test -p codegen type_map` — confirm GREEN (6 passed).
- [ ] Commit: `git commit -am "T3: implement type_map.rs with 6 tests"`

**Expected output:** `test result: ok. 6 passed`
**Verification:** `cargo test -p codegen type_map`

---

### T4 — Update `tools/codegen/src/main.rs` to use clap

```yaml
task:
  id: 4
  description: "Replace manual arg parsing with clap Parser; wire --field flags to all generators"
  skill: null
  parallel: false
  depends_on: [1, 2, 3]
```

- [ ] Replace `tools/codegen/src/main.rs` with:

```rust
mod api_generator;
mod app_generator;
mod domain_generator;
mod field_def;
mod helpers;
mod infra_generator;
mod migration_generator;
mod type_map;

use clap::Parser;
use field_def::{parse_field, FieldDef};
use helpers::{to_pascal_case, to_snake_case};
use std::path::Path;

#[derive(Parser)]
#[command(name = "codegen", about = "Scaffold a new entity across all 5 layers")]
struct Cli {
  entity: String,
  #[arg(long = "field", value_name = "name:Type[:modifier]")]
  fields: Vec<String>,
}

fn main() {
  let cli = Cli::parse();
  let fields: Vec<FieldDef> = cli
    .fields
    .iter()
    .map(|f| {
      parse_field(f).unwrap_or_else(|e| {
        eprintln!("Error parsing --field '{}': {}", f, e);
        std::process::exit(1);
      })
    })
    .collect();

  let entity = &cli.entity;
  let output_dir = Path::new(".");

  println!("Generating scaffold for: {}", entity);

  domain_generator::generate(entity, &fields, output_dir);
  app_generator::generate(entity, &fields, output_dir);
  infra_generator::generate(entity, &fields, output_dir);
  api_generator::generate(entity, &fields, output_dir);
  migration_generator::generate(entity, &fields, output_dir);

  let pascal = to_pascal_case(entity);
  let snake = to_snake_case(&pascal);
  println!("\n=== REGISTRATION CHECKLIST ===");
  println!("packages/domain/src/lib.rs              → pub mod {snake}; (in aggregate, commands, queries)");
  println!("packages/infrastructure/src/entities/mod.rs → pub mod {snake};");
  println!("packages/infrastructure/src/mappers/mod.rs  → pub mod {snake};");
  println!("packages/api/src/endpoints/mod.rs           → register {snake}::register(state.clone())");
  println!("packages/database/src/migrations/mod.rs     → register the generated migration");
  println!("\nDone! Register the files above, then run: cargo build");
}
```

> **Note:** `migration_generator` module doesn't exist yet. Create an empty stub `tools/codegen/src/migration_generator.rs` with `pub fn generate(_: &str, _: &[crate::field_def::FieldDef], _: &std::path::Path) {}` so it compiles. T8 will implement it fully.

- [ ] Verify backward compat: `cargo run -p codegen -- Member` — still generates stubs without error.
- [ ] Verify new CLI: `cargo run -p codegen -- --help` — shows `--field` option.
- [ ] Run `cargo test -p codegen` — existing 10 tests still pass (generators still use old signatures; T5–T9 will update them).

> **Note for T5–T9:** The generators still use the old `generate(entity, output_dir)` signature at this point. Update `main.rs` calls to pass `&fields` only after each generator's signature is updated in its own task. Use temporary empty-vec calls until each generator is ready.

- [ ] Commit: `git commit -am "T4: replace manual args with clap CLI"`

**Expected output:** `cargo run -p codegen -- --help` shows `--field` option; `cargo test -p codegen` → 10 passed
**Verification:** `cargo test -p codegen && cargo run -p codegen -- --help`

---

### T5 — Update `domain_generator.rs` — field-aware aggregate, commands, queries

```yaml
task:
  id: 5
  description: "domain_generator::generate now takes &[FieldDef]; emits real field declarations"
  skill: test-driven-development
  parallel: true
  depends_on: [4]
  blockers: []
```

> Parallel group T5–T9: each modifies a different file. No git worktrees needed.

- [ ] Add a failing test to `domain_generator.rs`:

```rust
#[test]
fn aggregate_contains_declared_fields() {
  use crate::field_def::{FieldDef, FieldType};
  let tmp = std::env::temp_dir().join("codegen_v2_domain");
  let fields = vec![
    FieldDef { name: "username".into(), ty: FieldType::Str, unique: false, nullable: false },
    FieldDef { name: "age".into(),      ty: FieldType::I32, unique: false, nullable: true  },
  ];
  generate("member", &fields, &tmp);
  let content = std::fs::read_to_string(
    tmp.join("packages/domain/src/aggregate/member.rs")
  ).unwrap();
  assert!(content.contains("pub username: String"), "missing username field");
  assert!(content.contains("pub age: Option<i32>"), "missing age field");
  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn create_command_contains_all_fields_non_optional() {
  use crate::field_def::{FieldDef, FieldType};
  let tmp = std::env::temp_dir().join("codegen_v2_domain_cmd");
  let fields = vec![
    FieldDef { name: "title".into(), ty: FieldType::Str, unique: false, nullable: true },
  ];
  generate("post", &fields, &tmp);
  let content = std::fs::read_to_string(
    tmp.join("packages/domain/src/commands/post.rs")
  ).unwrap();
  assert!(content.contains("pub title: String"), "CreatePostCommand must have non-optional title");
  std::fs::remove_dir_all(&tmp).ok();
}
```

- [ ] Run `cargo test -p codegen domain_generator` — RED (signature mismatch).
- [ ] Update `domain_generator.rs` signature and body. Key changes:

```rust
use crate::field_def::FieldDef;

pub fn generate(entity: &str, fields: &[FieldDef], output_dir: &std::path::Path) {
  // struct fields for aggregate
  let struct_fields: String = if fields.is_empty() {
    "  // TODO: add fields\n".into()
  } else {
    fields.iter()
      .map(|f| format!("  pub {}: {},\n", f.name, f.ty.rust_type(f.nullable)))
      .collect()
  };

  // fields for CreateCommand — always non-optional (required at creation)
  let create_fields: String = if fields.is_empty() {
    "  // TODO: add fields\n".into()
  } else {
    fields.iter()
      .map(|f| format!("  pub {}: {},\n", f.name, f.ty.rust_type(false)))
      .collect()
  };

  // fields for UpdateCommand — all Option<BaseType>
  let update_fields: String = if fields.is_empty() {
    "  // TODO: add optional update fields\n".into()
  } else {
    fields.iter()
      .map(|f| format!("  pub {}: Option<{}>,\n", f.name, f.ty.rust_type(false)))
      .collect()
  };

  // aggregate template — replace old `// TODO: add domain fields` with struct_fields
  // commands template — replace old TODO sections with create_fields / update_fields
  // queries template — unchanged (no field dependency)
  // ... rest of generate body unchanged except adding fields param
}
```

Also update the existing tests in this file to pass `&[]` as the fields argument.

- [ ] Update `main.rs` to pass `&fields` to `domain_generator::generate`.
- [ ] Run `cargo test -p codegen domain_generator` — GREEN (original tests + 2 new = pass).
- [ ] Commit: `git commit -am "T5: domain_generator field-aware"`

**Expected output:** original domain tests + 2 new tests pass
**Verification:** `cargo test -p codegen domain_generator`

---

### T6 — Update `app_generator.rs` — all 7 handler files, uniform `AppError`

```yaml
task:
  id: 6
  description: "app_generator generates all 7 handler files; update_handler with field-apply loop; AppError throughout"
  skill: test-driven-development
  parallel: true
  depends_on: [4]
```

- [ ] Add failing tests to `app_generator.rs`:

```rust
#[test]
fn generates_all_seven_handler_files() {
  use crate::field_def::{FieldDef, FieldType};
  let tmp = std::env::temp_dir().join("codegen_v2_app_all");
  let fields = vec![
    FieldDef { name: "title".into(), ty: FieldType::Str, unique: false, nullable: false },
  ];
  generate("post", &fields, &tmp);
  let base = tmp.join("packages/application/src/post");
  for f in &[
    "command_handlers/create_handler.rs",
    "command_handlers/update_handler.rs",
    "command_handlers/delete_handler.rs",
    "query_handlers/get_all_handler.rs",
    "query_handlers/get_by_id_handler.rs",
    "query_handlers/list_handler.rs",
    "service.rs",
  ] {
    assert!(base.join(f).exists(), "missing: {}", f);
  }
  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn update_handler_contains_field_apply_loop() {
  use crate::field_def::{FieldDef, FieldType};
  let tmp = std::env::temp_dir().join("codegen_v2_app_update");
  let fields = vec![
    FieldDef { name: "username".into(), ty: FieldType::Str, unique: false, nullable: false },
    FieldDef { name: "age".into(),      ty: FieldType::I32, unique: false, nullable: false },
  ];
  generate("member", &fields, &tmp);
  let content = std::fs::read_to_string(
    tmp.join("packages/application/src/member/command_handlers/update_handler.rs")
  ).unwrap();
  assert!(content.contains("cmd.username"), "update_handler must apply username");
  assert!(content.contains("cmd.age"), "update_handler must apply age");
  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn service_trait_uses_app_error() {
  use crate::field_def::FieldDef;
  let tmp = std::env::temp_dir().join("codegen_v2_app_svc");
  generate("post", &[], &tmp);
  let content = std::fs::read_to_string(
    tmp.join("packages/application/src/post/service.rs")
  ).unwrap();
  assert!(content.contains("AppError"), "service trait must use AppError not DbErr");
  assert!(!content.contains("DbErr"), "service trait must NOT use DbErr");
  std::fs::remove_dir_all(&tmp).ok();
}
```

- [ ] Run `cargo test -p codegen app_generator` — RED.
- [ ] Update `app_generator.rs`. Full list of generated files and key patterns:

**`service.rs`** — service trait, 6 methods, all `Result<_, AppError>`:
```rust
// trait method signatures:
async fn get_all(&self) -> Result<Vec<{Pascal}>, AppError>;
async fn get_by_id(&self, id: i32) -> Result<Option<{Pascal}>, AppError>;
async fn list(&self, page: u64, page_size: u64) -> Result<Paged<{Pascal}>, AppError>;
async fn create(&self, item: {Pascal}) -> Result<{Pascal}, AppError>;
async fn update(&self, item: {Pascal}) -> Result<{Pascal}, AppError>;
async fn delete(&self, id: i32) -> Result<Option<i32>, AppError>;
```

**`create_handler.rs`** — build aggregate from command, call `service.create`:
```rust
// Key body: build {Pascal} from cmd fields (or TODO if empty), call service.create
let item = {Pascal} {
  id: 0,
  {struct_build_fields}  // e.g., "username: cmd.username,\n  age: cmd.age,"
};
self.service.create(item).await
```
Where `struct_build_fields` = for each field: `format!("  {}: cmd.{},\n", f.name, f.name)`.

**`update_handler.rs`** — fetch-first, apply optional fields, call `service.update`:
```rust
let mut item = self.service.get_by_id(cmd.id).await?
  .ok_or_else(|| AppError::NotFound(format!("{pascal} {} not found", cmd.id)))?;
// field-apply loop:
{update_apply_lines}  // e.g., "if let Some(v) = cmd.username { item.username = v; }"
self.service.update(item).await
```
Where `update_apply_lines` = for each field: `format!("if let Some(v) = cmd.{name} {{ item.{name} = v; }}\n", name=f.name)`.
Empty fields: emit `// TODO: apply optional field updates`.

**`delete_handler.rs`** — calls `service.delete`, returns `AppError::NotFound` on `None`:
```rust
match self.service.delete(cmd.id).await? {
  Some(id) => Ok(id),
  None => Err(AppError::NotFound(format!("{pascal} {} not found", cmd.id))),
}
```

**`get_all_handler.rs`** — delegates to `service.get_all`.

**`get_by_id_handler.rs`** — delegates to `service.get_by_id`, maps `None → NotFound`.

**`list_handler.rs`** — delegates to `service.list(page.unwrap_or(1), page_size.unwrap_or(10))`.

Update all existing tests in this file to pass `&[]` as fields arg.

- [ ] Update `main.rs` to pass `&fields` to `app_generator::generate`.
- [ ] Run `cargo test -p codegen app_generator` — GREEN (original + 3 new tests pass).
- [ ] Commit: `git commit -am "T6: app_generator — all 7 handlers, uniform AppError"`

**Expected output:** all app_generator tests pass
**Verification:** `cargo test -p codegen app_generator`

---

### T7 — Update `api_generator.rs` — all 7 endpoints + complete `mod.rs`

```yaml
task:
  id: 7
  description: "api_generator generates all 7 endpoint files and a complete mod.rs route chain"
  skill: test-driven-development
  parallel: true
  depends_on: [4]
```

- [ ] Add failing tests to `api_generator.rs`:

```rust
#[test]
fn generates_all_seven_endpoint_files() {
  use crate::field_def::FieldDef;
  let tmp = std::env::temp_dir().join("codegen_v2_api_all");
  generate("post", &[], &tmp);
  let base = tmp.join("packages/api/src/endpoints/post");
  for f in &["create.rs", "update.rs", "delete.rs",
             "get_all.rs", "get_by_id.rs", "list.rs", "mod.rs"] {
    assert!(base.join(f).exists(), "missing: {}", f);
  }
  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn mod_rs_contains_full_route_chain() {
  use crate::field_def::FieldDef;
  let tmp = std::env::temp_dir().join("codegen_v2_api_mod");
  generate("post", &[], &tmp);
  let content = std::fs::read_to_string(
    tmp.join("packages/api/src/endpoints/post/mod.rs")
  ).unwrap();
  assert!(content.contains("create::route"), "mod.rs must wire create route");
  assert!(content.contains("update::route"), "mod.rs must wire update route");
  assert!(content.contains("delete::route"), "mod.rs must wire delete route");
  assert!(content.contains("pub fn register"), "mod.rs must export register fn");
  assert!(!content.contains("placeholder"), "mod.rs must not have placeholder route");
  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn update_endpoint_uses_path_param_injection() {
  use crate::field_def::FieldDef;
  let tmp = std::env::temp_dir().join("codegen_v2_api_update");
  generate("post", &[], &tmp);
  let content = std::fs::read_to_string(
    tmp.join("packages/api/src/endpoints/post/update.rs")
  ).unwrap();
  assert!(content.contains("warp::path!(\"post\" / i32)"), "update must use path param");
  assert!(content.contains("cmd.id = post_id"), "update must inject id into cmd");
  std::fs::remove_dir_all(&tmp).ok();
}
```

- [ ] Run `cargo test -p codegen api_generator` — RED.
- [ ] Update `api_generator.rs`. Key additions:

**`update.rs`** — `PUT /{snake}/:id`, body `Update{Pascal}Command`, inject `cmd.id = {snake}_id`:
```rust
warp::path!("{snake}" / i32)
  .and(warp::put())
  .and(with_handler(handler))
  .and(warp::body::json())
  .and_then(handle_update_{snake})
// handler fn:
async fn handle_update_{snake}<T>({snake}_id: i32, handler: ..., mut cmd: Update{Pascal}Command)
  -> Result<impl warp::Reply, warp::Rejection> {
  cmd.id = {snake}_id;
  match handler.handle(cmd).await {
    Ok(item) => Ok(warp::reply::json(&item)),
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }
}
```

**`delete.rs`** — `DELETE /{snake}/:id`, `ApiError::NotFound` on `None`:
```rust
warp::path!("{snake}" / i32)
  .and(warp::delete())
  .and(with_handler(handler))
  .and_then(handle_delete_{snake})
// handler returns ApiError::NotFound when service returns None
```

**`get_by_id.rs`** — `GET /{snake}/:id`, `ApiError::NotFound` on `None`.

**`list.rs`** — `GET /{snake}/list` with `warp::query::<List{Pascal}>()`.

**`mod.rs`** — full registration:
```rust
mod create; mod update; mod delete; mod get_all; mod get_by_id; mod list;

pub fn register(state: Arc<AppState>) -> impl Filter<...> + Clone {
  let repo = {Pascal}Repository::new(state.db.connection.clone());
  let service = Arc::new({Pascal}ServiceImpl::new(repo));
  let create_handler  = Arc::new(Create{Pascal}CommandHandler::new(service.clone()));
  let update_handler  = Arc::new(Update{Pascal}CommandHandler::new(service.clone()));
  let delete_handler  = Arc::new(Delete{Pascal}CommandHandler::new(service.clone()));
  let get_all_handler = Arc::new(GetAll{Pascal}QueryHandler::new(service.clone()));
  let get_by_id_handler = Arc::new(GetById{Pascal}QueryHandler::new(service.clone()));
  let list_handler    = Arc::new(List{Pascal}QueryHandler::new(service.clone()));

  create::route(create_handler)
    .or(update::route(update_handler))
    .or(delete::route(delete_handler))
    .or(get_all::route(get_all_handler))
    .or(get_by_id::route(get_by_id_handler))
    .or(list::route(list_handler))
}
```

Update existing test to pass `&[]` as fields arg.

- [ ] Update `main.rs` to pass `&fields` to `api_generator::generate`.
- [ ] Run `cargo test -p codegen api_generator` — GREEN (original + 3 new tests pass).
- [ ] Commit: `git commit -am "T7: api_generator — all 7 endpoints + complete mod.rs"`

**Expected output:** all api_generator tests pass
**Verification:** `cargo test -p codegen api_generator`

---

### T8 — Implement `tools/codegen/src/migration_generator.rs`

```yaml
task:
  id: 8
  description: "New migration_generator: ColumnDef per field, Iden enum, up/down impls"
  skill: test-driven-development
  parallel: true
  depends_on: [4]
```

- [ ] Add failing tests to `migration_generator.rs` (replacing the stub from T4):

```rust
#[cfg(test)]
mod tests {
  use super::*;
  use crate::field_def::{FieldDef, FieldType};

  #[test]
  fn generates_migration_file() {
    let tmp = std::env::temp_dir().join("codegen_v2_migration");
    let fields = vec![
      FieldDef { name: "username".into(), ty: FieldType::Str, unique: false, nullable: false },
      FieldDef { name: "score".into(),    ty: FieldType::I32, unique: false, nullable: true  },
    ];
    generate("member", &fields, &tmp);
    let dir = tmp.join("packages/database/src/migrations");
    let files: Vec<_> = std::fs::read_dir(&dir).unwrap()
      .filter_map(|e| e.ok())
      .collect();
    assert!(!files.is_empty(), "migration file must be created");
    let content = std::fs::read_to_string(files[0].path()).unwrap();
    assert!(content.contains("Username"), "must include Username Iden variant");
    assert!(content.contains(".string()"), "must include string() col method");
    assert!(content.contains(".not_null()"), "required field must be not_null");
    assert!(content.contains("Score"), "must include Score Iden variant");
    std::fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn unique_field_has_unique_key_in_migration() {
    let tmp = std::env::temp_dir().join("codegen_v2_migration_unique");
    let fields = vec![
      FieldDef { name: "email".into(), ty: FieldType::Str, unique: true, nullable: false },
    ];
    generate("user", &fields, &tmp);
    let dir = tmp.join("packages/database/src/migrations");
    let files: Vec<_> = std::fs::read_dir(&dir).unwrap()
      .filter_map(|e| e.ok()).collect();
    let content = std::fs::read_to_string(files[0].path()).unwrap();
    assert!(content.contains(".unique_key()"), "unique field must have unique_key()");
    std::fs::remove_dir_all(&tmp).ok();
  }
}
```

- [ ] Run `cargo test -p codegen migration_generator` — RED.
- [ ] Implement `migration_generator.rs`:

```rust
use crate::field_def::FieldDef;
use crate::helpers::{to_pascal_case, to_snake_case};
use chrono::Local;
use std::{fs, path::Path};

pub fn generate(entity: &str, fields: &[FieldDef], output_dir: &Path) {
  let pascal = to_pascal_case(entity);
  let snake  = to_snake_case(&pascal);
  let table_name = format!("{}s", snake);  // pluralize naively
  let timestamp = Local::now().format("%Y%m%d%H%M%S");
  let file_name = format!("m{}_create_{}_table.rs", timestamp, snake);

  // Iden enum variants: Table, Id, + one per field (PascalCase)
  let iden_variants: String = fields.iter()
    .map(|f| format!("  {},\n", to_pascal_case(&f.name)))
    .collect();

  // col defs for each user-defined field
  let col_defs: String = fields.iter().map(|f| {
    let variant = to_pascal_case(&f.name);
    let type_method = f.ty.seaorm_col_method();
    let mut chain = format!(
      "          .col(ColumnDef::new({pascal}::{variant}).{type_method}",
      pascal=pascal, variant=variant, type_method=type_method
    );
    if !f.nullable { chain.push_str(".not_null()"); }
    if f.unique    { chain.push_str(".unique_key()"); }
    chain.push_str(")\n");
    chain
  }).collect();

  let content = format!(
r#"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {{
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
    manager
      .create_table(
        Table::create()
          .table("{table_name}")
          .if_not_exists()
          .col(
            ColumnDef::new({pascal}::Id)
              .integer()
              .not_null()
              .primary_key()
              .auto_increment(),
          )
{col_defs}          .to_owned(),
      )
      .await
  }}

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
    manager
      .drop_table(Table::drop().table({pascal}::Table).to_owned())
      .await
  }}
}}

#[derive(Iden)]
enum {pascal} {{
  Table,
  Id,
{iden_variants}}}
"#,
    table_name=table_name, pascal=pascal, col_defs=col_defs, iden_variants=iden_variants
  );

  let migration_dir = output_dir.join("packages/database/src/migrations");
  fs::create_dir_all(&migration_dir).unwrap();
  fs::write(migration_dir.join(&file_name), content).unwrap();

  println!("  [database] packages/database/src/migrations/{}", file_name);
}
```

- [ ] Run `cargo test -p codegen migration_generator` — GREEN (2 tests pass).
- [ ] Commit: `git commit -am "T8: implement migration_generator.rs with 2 tests"`

**Expected output:** `test result: ok. 2 passed`
**Verification:** `cargo test -p codegen migration_generator`

---

### T9 — Update `infra_generator.rs` — entity + mapper

```yaml
task:
  id: 9
  description: "infra_generator generates SeaORM entity (DeriveEntityModel, Column enum) and both mapper From impls"
  skill: test-driven-development
  parallel: true
  depends_on: [4]
```

- [ ] Add failing tests to `infra_generator.rs`:

```rust
#[test]
fn generates_seaorm_entity_with_model_fields() {
  use crate::field_def::{FieldDef, FieldType};
  let tmp = std::env::temp_dir().join("codegen_v2_infra_entity");
  let fields = vec![
    FieldDef { name: "username".into(), ty: FieldType::Str, unique: false, nullable: false },
    FieldDef { name: "score".into(),    ty: FieldType::I32, unique: false, nullable: true  },
  ];
  generate("member", &fields, &tmp);
  let content = std::fs::read_to_string(
    tmp.join("packages/infrastructure/src/entities/member.rs")
  ).unwrap();
  assert!(content.contains("DeriveEntityModel"), "must derive DeriveEntityModel");
  assert!(content.contains("pub username: String"), "must have username field");
  assert!(content.contains("pub score: Option<i32>"), "nullable field must be Option");
  std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn generates_mapper_with_both_from_impls() {
  use crate::field_def::{FieldDef, FieldType};
  let tmp = std::env::temp_dir().join("codegen_v2_infra_mapper");
  let fields = vec![
    FieldDef { name: "username".into(), ty: FieldType::Str, unique: false, nullable: false },
  ];
  generate("member", &fields, &tmp);
  let content = std::fs::read_to_string(
    tmp.join("packages/infrastructure/src/mappers/member.rs")
  ).unwrap();
  assert!(content.contains("impl From<member::Model> for Member"), "must have Model->Aggregate impl");
  assert!(content.contains("impl From<Member> for member::ActiveModel"), "must have Aggregate->ActiveModel impl");
  assert!(content.contains("not_set()"), "id==0 must use not_set()");
  assert!(content.contains("ActiveValue::Set(item.username)"), "field must be wrapped in Set");
  std::fs::remove_dir_all(&tmp).ok();
}
```

- [ ] Run `cargo test -p codegen infra_generator` — RED.
- [ ] Update `infra_generator.rs` to add `fields: &[FieldDef]` param and generate entity + mapper sections.

**Entity template** (replace existing repository-only body):

```rust
// Entity file content:
let col_enum_variants: String = fields.iter()
  .map(|f| format!("  {},\n", to_pascal_case(&f.name)))
  .collect();
let model_fields: String = if fields.is_empty() {
  "  // TODO: add fields\n".into()
} else {
  fields.iter()
    .map(|f| format!("  pub {}: {},\n", f.name, f.ty.rust_type(f.nullable)))
    .collect()
};
// Template:
// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
// #[sea_orm(table_name = "{snake}s")]
// pub struct Model { #[sea_orm(primary_key)] pub id: i32, {model_fields} }
// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)] pub enum Relation {}
// impl ActiveModelBehavior for ActiveModel {}
```

**Mapper template** (from existing `book.rs` mapper — generalize):

```rust
// From<{snake}::Model> for {Pascal}:
let model_to_agg: String = fields.iter()
  .map(|f| format!("      {}: model.{},\n", f.name, f.name))
  .collect();

// From<{Pascal}> for {snake}::ActiveModel:
let agg_to_active: String = fields.iter()
  .map(|f| format!("      {}: ActiveValue::Set(item.{}),\n", f.name, f.name))
  .collect();
// id uses: if item.id == 0 { ActiveValue::not_set() } else { ActiveValue::Set(item.id) }
```

Update existing test to pass `&[]`.

- [ ] Run `cargo test -p codegen infra_generator` — GREEN (original + 2 new tests pass).
- [ ] Commit: `git commit -am "T9: infra_generator — entity + mapper with fields"`

**Expected output:** original test + 2 new entity/mapper tests pass
**Verification:** `cargo test -p codegen infra_generator`

---

### T10 — Update `infra_generator.rs` — full CRUD repository

```yaml
task:
  id: 10
  description: "infra_generator adds full repository body: find_all, find_by_id, insert, update, delete, list (Paged)"
  skill: test-driven-development
  parallel: false
  depends_on: [9]
```

- [ ] Add failing test:

```rust
#[test]
fn generates_full_repository_with_all_crud_methods() {
  use crate::field_def::FieldDef;
  let tmp = std::env::temp_dir().join("codegen_v2_infra_repo");
  generate("member", &[], &tmp);
  let content = std::fs::read_to_string(
    tmp.join("packages/infrastructure/src/repositories/member_repository.rs")
  ).unwrap();
  for method in &["find_all", "find_by_id", "insert", "update", "delete", "list_member"] {
    assert!(content.contains(method), "must contain method: {}", method);
  }
  assert!(content.contains("Paged"), "list must return Paged<T>");
  assert!(content.contains("div_ceil"), "pagination must use div_ceil");
  std::fs::remove_dir_all(&tmp).ok();
}
```

- [ ] Run `cargo test -p codegen infra_generator::tests::generates_full_repository_with_all_crud_methods` — RED.
- [ ] Add full repository generation to `infra_generator.rs`. The repository template mirrors `book_repository.rs`:

```rust
// Repository template — entity-agnostic, follows BookRepository pattern exactly:
// pub struct {Pascal}Repository { db: DatabaseConnection }
// impl {Pascal}Repository {
//   pub fn new(db: DatabaseConnection) -> Self { Self { db } }
//   pub async fn find_all(&self) -> Result<Vec<{snake}::Model>, DbErr>
//   pub async fn list_{snake}(&self, page: u64, page_size: u64) -> Result<Paged<{snake}::Model>, DbErr>
//     page_size clamped 1-100, offset = (page-1)*page_size, count then fetch, div_ceil
//   pub async fn find_by_id(&self, id: i32) -> Result<Option<{snake}::Model>, DbErr>
//   pub async fn insert(&self, model: {snake}::ActiveModel) -> Result<{snake}::Model, DbErr>
//   pub async fn update(&self, model: {snake}::ActiveModel) -> Result<{snake}::Model, DbErr>
//   pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr>
// }
// Uses: sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
//                 PaginatorTrait, QuerySelect}
```

- [ ] Run `cargo test -p codegen infra_generator` — GREEN (all prior tests + 1 new).
- [ ] Commit: `git commit -am "T10: infra_generator — full CRUD repository"`

**Expected output:** all infra_generator tests pass
**Verification:** `cargo test -p codegen infra_generator`

---

### T11 — Update `infra_generator.rs` — service impl

```yaml
task:
  id: 11
  description: "infra_generator adds {Pascal}ServiceImpl wiring all 6 repository methods, uniform AppError"
  skill: test-driven-development
  parallel: false
  depends_on: [10]
```

- [ ] Add failing test:

```rust
#[test]
fn generates_service_impl_with_app_error() {
  use crate::field_def::FieldDef;
  let tmp = std::env::temp_dir().join("codegen_v2_infra_svc");
  generate("member", &[], &tmp);
  let content = std::fs::read_to_string(
    tmp.join("packages/infrastructure/src/services/member_service.rs")
  ).unwrap();
  assert!(content.contains("impl MemberService for MemberServiceImpl"), "must impl trait");
  assert!(content.contains("AppError"), "must use AppError");
  assert!(!content.contains("todo!()"), "service impl must not have todo!()");
  for method in &["get_all", "get_by_id", "list", "create", "update", "delete"] {
    assert!(content.contains(method), "must implement: {}", method);
  }
  std::fs::remove_dir_all(&tmp).ok();
}
```

- [ ] Run `cargo test -p codegen infra_generator::tests::generates_service_impl_with_app_error` — RED.
- [ ] Add service impl generation to `infra_generator.rs`. Template mirrors `book_service.rs` without `borrow_book`/`return_book`:

```rust
// Service impl template:
// pub struct {Pascal}ServiceImpl { repo: {Pascal}Repository }
// impl {Pascal}ServiceImpl { pub fn new(repo: {Pascal}Repository) -> Self { Self { repo } } }
// #[async_trait]
// impl {Pascal}Service for {Pascal}ServiceImpl {
//   async fn get_all(&self) -> Result<Vec<{Pascal}>, AppError> {
//     self.repo.find_all().await
//       .map(|items| items.into_iter().map(Into::into).collect())
//       .map_err(|e| AppError::DatabaseError(e.to_string()))
//   }
//   async fn get_by_id(&self, id: i32) -> Result<Option<{Pascal}>, AppError> {
//     self.repo.find_by_id(id).await
//       .map(|opt| opt.map(Into::into))
//       .map_err(|e| AppError::DatabaseError(e.to_string()))
//   }
//   async fn list(&self, page, page_size) -> Result<Paged<{Pascal}>, AppError> {
//     let page = self.repo.list_{snake}(page, page_size).await
//       .map_err(|e| AppError::DatabaseError(e.to_string()))?;
//     Ok(Paged { items: page.items.into_iter().map(Into::into).collect(), ..page fields })
//   }
//   async fn create(&self, item) -> Result<{Pascal}, AppError> {
//     self.repo.insert(item.into()).await.map(Into::into)
//       .map_err(|e| AppError::DatabaseError(e.to_string()))
//   }
//   async fn update(&self, item) -> Result<{Pascal}, AppError> {
//     self.repo.update(item.into()).await.map(Into::into)
//       .map_err(|e| AppError::DatabaseError(e.to_string()))
//   }
//   async fn delete(&self, id: i32) -> Result<Option<i32>, AppError> {
//     let result = self.repo.delete(id).await
//       .map_err(|e| AppError::DatabaseError(e.to_string()))?;
//     Ok(if result.rows_affected == 0 { None } else { Some(id) })
//   }
// }
```

- [ ] Run `cargo test -p codegen infra_generator` — GREEN (all tests pass).
- [ ] Commit: `git commit -am "T11: infra_generator — service impl with uniform AppError"`

**Expected output:** all infra_generator tests pass
**Verification:** `cargo test -p codegen infra_generator`

---

### T12 — Integration test: all 22 files generated for entity with fields

```yaml
task:
  id: 12
  description: "Integration test: cargo run with --field flags generates all 22 files with correct symbols"
  skill: test-driven-development
  parallel: false
  depends_on: [5, 6, 7, 8, 9, 10, 11]
```

- [ ] Add integration test to `tools/codegen/src/main.rs` (or a new test file `tests/integration.rs`):

```rust
#[cfg(test)]
mod integration_tests {
  use crate::{
    api_generator, app_generator, domain_generator,
    field_def::{FieldDef, FieldType},
    infra_generator, migration_generator,
  };
  use std::{env, fs};

  fn sample_fields() -> Vec<FieldDef> {
    vec![
      FieldDef { name: "username".into(), ty: FieldType::Str,  unique: true,  nullable: false },
      FieldDef { name: "age".into(),      ty: FieldType::I32,  unique: false, nullable: true  },
      FieldDef { name: "active".into(),   ty: FieldType::Bool, unique: false, nullable: false },
    ]
  }

  #[test]
  fn all_22_files_generated_for_member() {
    let tmp = env::temp_dir().join("codegen_v2_integration");
    let fields = sample_fields();
    domain_generator::generate("member", &fields, &tmp);
    app_generator::generate("member", &fields, &tmp);
    infra_generator::generate("member", &fields, &tmp);
    api_generator::generate("member", &fields, &tmp);
    migration_generator::generate("member", &fields, &tmp);

    let expected = vec![
      "packages/domain/src/aggregate/member.rs",
      "packages/domain/src/commands/member.rs",
      "packages/domain/src/queries/member.rs",
      "packages/infrastructure/src/entities/member.rs",
      "packages/infrastructure/src/mappers/member.rs",
      "packages/infrastructure/src/repositories/member_repository.rs",
      "packages/infrastructure/src/services/member_service.rs",
      "packages/application/src/member/service.rs",
      "packages/application/src/member/command_handlers/create_handler.rs",
      "packages/application/src/member/command_handlers/update_handler.rs",
      "packages/application/src/member/command_handlers/delete_handler.rs",
      "packages/application/src/member/query_handlers/get_all_handler.rs",
      "packages/application/src/member/query_handlers/get_by_id_handler.rs",
      "packages/application/src/member/query_handlers/list_handler.rs",
      "packages/api/src/endpoints/member/create.rs",
      "packages/api/src/endpoints/member/update.rs",
      "packages/api/src/endpoints/member/delete.rs",
      "packages/api/src/endpoints/member/get_all.rs",
      "packages/api/src/endpoints/member/get_by_id.rs",
      "packages/api/src/endpoints/member/list.rs",
      "packages/api/src/endpoints/member/mod.rs",
    ];
    for path in &expected {
      assert!(tmp.join(path).exists(), "missing: {}", path);
    }

    // migration file exists (timestamp in name, so glob)
    let migration_dir = tmp.join("packages/database/src/migrations");
    let migration_files: Vec<_> = fs::read_dir(&migration_dir).unwrap()
      .filter_map(|e| e.ok()).collect();
    assert_eq!(migration_files.len(), 1, "exactly 1 migration file expected");

    // spot-check field propagation
    let agg = fs::read_to_string(tmp.join("packages/domain/src/aggregate/member.rs")).unwrap();
    assert!(agg.contains("pub username: String"));
    assert!(agg.contains("pub age: Option<i32>"));
    assert!(agg.contains("pub active: bool"));

    let svc = fs::read_to_string(tmp.join("packages/infrastructure/src/services/member_service.rs")).unwrap();
    assert!(svc.contains("impl MemberService for MemberServiceImpl"));

    let mod_rs = fs::read_to_string(tmp.join("packages/api/src/endpoints/member/mod.rs")).unwrap();
    assert!(mod_rs.contains("pub fn register"));
    assert!(!mod_rs.contains("placeholder"));

    fs::remove_dir_all(&tmp).ok();
  }
}
```

- [ ] Run `cargo test -p codegen integration_tests` — confirm GREEN.
- [ ] Commit: `git commit -am "T12: integration test — all 22 files generated correctly"`

**Expected output:** integration test passes
**Verification:** `cargo test -p codegen integration_tests`

---

### T13 — Backward compat + full suite verification

```yaml
task:
  id: 13
  description: "Confirm: no --field still generates stubs; full suite (all tests) passes"
  skill: null
  parallel: false
  depends_on: [12]
```

- [ ] Run: `cargo run -p codegen -- TestEntity` (no `--field` flags) — confirm stubs generated with `// TODO` markers, no crash.
- [ ] Run: `cargo run -p codegen -- Member --field username:String --field age:i32` — confirm output lists 22 generated file paths.
- [ ] Run: `cargo test -p codegen` — confirm ALL tests pass (original 10 + new tests from T2–T12).
- [ ] Run: `cargo test --workspace` — confirm all 44+ workspace tests still pass.
- [ ] Commit: `git commit -am "T13: backward compat verified, full suite passes"`

**Expected output:**
- `cargo test -p codegen` → all codegen tests pass (≥25 total)
- `cargo test --workspace` → all workspace tests pass (≥44 total)

**Verification:**
```
cargo test -p codegen
cargo test --workspace
cargo run -p codegen -- Member --field username:String --field age:i32 2>&1 | grep "\[api\]"
```

---

## Risks

| Risk | Mitigation |
|------|------------|
| Template strings with `{}` and `{{}}` escaping bugs in `format!` | Test each generator independently; run `cargo test -p codegen` after every task |
| `type_map` returns wrong SeaORM method name causing compile errors in generated code | Verify against `book_repository.rs` and `book.rs` entity patterns |
| Parallel tasks (T5–T9) modifying different files — no conflicts expected | Each task writes to a distinct file path; no worktrees needed |
| `chrono::Local::now()` produces duplicate timestamps in rapid test runs | Tests check file existence not exact name; acceptable for a dev tool |

---

## Execution Options

Plan complete and saved to `docs/plans/2026-11-07-codegen-v2-plan.md`.

> **1. Subagent-Driven (recommended)** — fresh agent per task with two-stage review
> (spec compliance then code quality), fast iteration.
>
> **2. Inline Execution** — execute tasks in this session using `executing-plans` skill,
> with human checkpoints between batches.
>
> Which approach?
