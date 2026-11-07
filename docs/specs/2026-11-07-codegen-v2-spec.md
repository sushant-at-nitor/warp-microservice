# Spec: Codegen v2 — Field-Aware Scaffold Generator (2026-11-07)

**Goal:** Extend `tools/codegen` to accept `--field name:Type[:modifier]` CLI flags and generate a complete, compilable 22-file scaffold across all 5 layers of the clean architecture.

**Approach:** CLI Field DSL — fields declared as `--field name:Type` flags on the command line; `clap` parses multi-value flags.

**Out of scope:** mod.rs auto-injection, domain events, auth middleware, specialized query handlers (get_available, search), interactive mode.

---

## CLI Interface

```bash
cargo run -p codegen -- <EntityName> [--field <name>:<type>[:<modifier>]...]

# Example
cargo run -p codegen -- Member \
  --field username:String \
  --field email:String:unique \
  --field age:i32
```

`clap` (derive feature) replaces the current `std::env::args()` manual parsing.
`id: i32` (PK, auto-increment) is always generated and must NOT be specified via `--field`.

---

## Field DSL

| DSL syntax | Rust type | SeaORM ColumnType |
|-----------|-----------|-------------------|
| `name:String` | `String` | `String(StringLen::None)` |
| `name:i32` | `i32` | `Integer` |
| `name:i64` | `i64` | `BigInteger` |
| `name:bool` | `bool` | `Boolean` |
| `name:f64` | `f64` | `Double` |
| `name:DateTime` | `DateTimeUtc` | `TimestampWithTimeZone` |

**Modifiers** (appended after type with `:`):
- `:unique` — adds `.unique_key()` on the SeaORM `ColumnDef` in migration
- `:required` — non-nullable column (default: nullable, field wrapped in `Option<T>`)

Unknown type → print error + `std::process::exit(1)`.

---

## Generated Files (22 total)

### Domain (3 files)
- `packages/domain/src/aggregate/{snake}.rs` — struct with all declared fields (not stubs)
- `packages/domain/src/commands/{snake}.rs` — `Create{Pascal}Command` with all fields, `Update{Pascal}Command` with all `Option<T>` fields + `#[serde(default)] id`, `Delete{Pascal}Command`
- `packages/domain/src/queries/{snake}.rs` — `GetAll`, `GetById`, `List{Pascal}` (page/page_size)

### Infrastructure (4 files)
- `packages/infrastructure/src/entities/{snake}.rs` — `#[derive(DeriveEntityModel)]`, `Model` struct mirroring fields, `Relation` enum, `ActiveModelBehavior` impl
- `packages/infrastructure/src/mappers/{snake}.rs` — `From<Model> for {Pascal}` and `From<{Pascal}> for ActiveModel` (with `id == 0 → not_set()` pattern)
- `packages/infrastructure/src/repositories/{snake}_repository.rs` — full CRUD: `find_all`, `find_by_id`, `insert`, `update`, `delete`, `list` (paginated, `Paged<T>`, page-size clamped 1–100)
- `packages/infrastructure/src/services/{snake}_service.rs` — `{Pascal}ServiceImpl` wiring all 6 repository methods, uniform `AppError` error type

### Application (7 files)
- `packages/application/src/{snake}/service.rs` — `{Pascal}Service` trait, 6 methods, all returning `Result<_, AppError>`
- `packages/application/src/{snake}/command_handlers/create_handler.rs` — builds aggregate from command, calls `service.create`
- `packages/application/src/{snake}/command_handlers/update_handler.rs` — fetch-first, apply optional field updates, calls `service.update`; returns `AppError::NotFound` if missing
- `packages/application/src/{snake}/command_handlers/delete_handler.rs` — calls `service.delete`, propagates `AppError::NotFound`
- `packages/application/src/{snake}/query_handlers/get_all_handler.rs` — delegates to `service.get_all`
- `packages/application/src/{snake}/query_handlers/get_by_id_handler.rs` — delegates to `service.get_by_id`, maps `None → AppError::NotFound`
- `packages/application/src/{snake}/query_handlers/list_handler.rs` — delegates to `service.list` with defaults (`page=1`, `page_size=10`)

### API (7 files)
- `packages/api/src/endpoints/{snake}/create.rs` — `POST /{snake}`, body `Create{Pascal}Command`
- `packages/api/src/endpoints/{snake}/update.rs` — `PUT /{snake}/:id`, path param injected as `cmd.id`
- `packages/api/src/endpoints/{snake}/delete.rs` — `DELETE /{snake}/:id`, `ApiError::NotFound` on `None`
- `packages/api/src/endpoints/{snake}/get_all.rs` — `GET /{snake}`
- `packages/api/src/endpoints/{snake}/get_by_id.rs` — `GET /{snake}/:id`, `ApiError::NotFound` on `None`
- `packages/api/src/endpoints/{snake}/list.rs` — `GET /{snake}/list?page=&page_size=`
- `packages/api/src/endpoints/{snake}/mod.rs` — full `.or()` route chain, all 6 handlers instantiated from `Arc<AppState>`

### Database (1 file)
- `packages/database/src/migrations/m{timestamp}_create_{snake}_table.rs` — `DeriveMigrationName`, `ColumnDef` per field with type + modifiers, `up`/`down` impls

### Stdout (not a file)
A registration checklist is printed listing every `mod.rs` file that must be edited and what line to add.

---

## Internal Architecture of `tools/codegen`

### New modules
- `src/field_def.rs` — `FieldDef { name: String, ty: FieldType, unique: bool, nullable: bool }`, `FieldType` enum, `parse_field(s: &str) -> Result<FieldDef, String>`
- `src/type_map.rs` — `FieldType::rust_type() -> &str`, `FieldType::seaorm_column() -> &str`
- `src/migration_generator.rs` — generates migration file from `&[FieldDef]`

### Modified modules
- `src/main.rs` — replace manual arg parsing with `clap::Parser` derive; pass `fields: Vec<FieldDef>` to all generators
- `src/domain_generator.rs` — `generate(entity, fields, output_dir)` — emit real field declarations
- `src/app_generator.rs` — `generate(entity, fields, output_dir)` — emit all 7 handler files, uniform `AppError`
- `src/infra_generator.rs` — `generate(entity, fields, output_dir)` — emit entity + mapper + full repository + service impl
- `src/api_generator.rs` — `generate(entity, fields, output_dir)` — emit all 7 endpoint files + complete `mod.rs`

### Backward compatibility
When invoked with no `--field` flags, all generators fall back to `// TODO: add fields` stub behavior. Existing 10 tests pass unmodified.

---

## Error Handling

| Condition | Behavior |
|-----------|----------|
| Unknown type in `--field` | `eprintln!` + `exit(1)` |
| `id` specified as a field | `eprintln!("id is reserved")` + `exit(1)` |
| Entity name missing | clap prints usage + exits |
| File write failure | `unwrap()` — dev tool, panic acceptable |

---

## Testing Strategy

- **`field_def.rs` unit tests**: parse valid DSL strings, reject unknown types, reject `id` field, handle modifiers
- **Per-generator unit tests**: supply 2–3 `FieldDef`s, assert field names and types appear in generated output
- **Migration generator unit tests**: assert column names and types appear
- **Backward compat test**: `generate("member", &[], dir)` → same stub output as today
- **Integration test**: `cargo run -p codegen -- Member --field username:String --field age:i32` → assert all 22 files exist and contain key symbols (`MemberRepository`, `impl MemberService for MemberServiceImpl`, etc.)
- Existing 10 tests continue to pass

---

## Dependency addition

```toml
# tools/codegen/Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }
chrono = "0.4"  # for migration timestamp
```
