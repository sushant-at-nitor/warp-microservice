use std::{fs, path::Path};

use crate::field_def::FieldDef;
use crate::helpers::{to_pascal_case, to_snake_case};

pub fn generate(entity: &str, _fields: &[FieldDef], output_dir: &Path) {
  let pascal = to_pascal_case(entity);
  let snake = to_snake_case(&pascal);

  let create_endpoint = format!(
    r#"use std::sync::Arc;
use warp::Filter;
use application::{snake}::{{
  {pascal}Service, Create{pascal}CommandHandler, Create{pascal}CommandHandlerTrait,
}};
use domain::commands::Create{pascal}Command;
use crate::errors::ApiError;

pub fn route<T>(
  handler: Arc<Create{pascal}CommandHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where T: {pascal}Service + Send + Sync + 'static,
{{
  warp::path!("{snake}")
    .and(warp::post())
    .and(warp::any().map(move || handler.clone()))
    .and(warp::body::json())
    .and_then(|handler: Arc<Create{pascal}CommandHandler<T>>, cmd: Create{pascal}Command| async move {{
      match handler.handle(cmd).await {{
        Ok(item) => Ok(warp::reply::json(&item)),
        Err(e) => Err(warp::reject::custom(ApiError::from(e))),
      }}
    }})
}}
"#,
    pascal = pascal,
    snake = snake
  );

  let update_endpoint = format!(
    r#"use crate::errors::ApiError;
use std::sync::Arc;
use warp::Filter;
use application::{snake}::{{
  {pascal}Service, Update{pascal}CommandHandler, Update{pascal}CommandHandlerTrait,
}};
use domain::commands::Update{pascal}Command;

pub fn route<T>(
  handler: Arc<Update{pascal}CommandHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  warp::path!("{snake}" / i32)
    .and(warp::put())
    .and(with_handler(handler))
    .and(warp::body::json())
    .and_then(handle_update_{snake})
}}

fn with_handler<T>(
  handler: Arc<Update{pascal}CommandHandler<T>>,
) -> impl Filter<
  Extract = (Arc<Update{pascal}CommandHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  warp::any().map(move || handler.clone())
}}

async fn handle_update_{snake}<T>(
  {snake}_id: i32,
  handler: Arc<Update{pascal}CommandHandler<T>>,
  mut cmd: Update{pascal}Command,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  cmd.id = {snake}_id;

  match handler.handle(cmd).await {{
    Ok(item) => Ok(warp::reply::json(&item)),
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }}
}}
"#,
    pascal = pascal,
    snake = snake
  );

  let delete_endpoint = format!(
    r#"use crate::errors::ApiError;
use std::sync::Arc;
use warp::Filter;
use application::{snake}::{{
  {pascal}Service, Delete{pascal}CommandHandler, Delete{pascal}CommandHandlerTrait,
}};
use domain::commands::Delete{pascal}Command;

pub fn route<T>(
  handler: Arc<Delete{pascal}CommandHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  warp::path!("{snake}" / i32)
    .and(warp::delete())
    .and(with_handler(handler))
    .and_then(handle_delete_{snake})
}}

fn with_handler<T>(
  handler: Arc<Delete{pascal}CommandHandler<T>>,
) -> impl Filter<
  Extract = (Arc<Delete{pascal}CommandHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  warp::any().map(move || handler.clone())
}}

async fn handle_delete_{snake}<T>(
  id: i32,
  handler: Arc<Delete{pascal}CommandHandler<T>>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  match handler.handle(Delete{pascal}Command {{ id }}).await {{
    Ok(item) => match item {{
      Some(item) => Ok(warp::reply::json(&item)),
      None => Err(warp::reject::custom(ApiError::NotFound(format!(
        "{pascal} {{}} not found",
        id
      )))),
    }},
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }}
}}
"#,
    pascal = pascal,
    snake = snake
  );

  let get_all_endpoint = format!(
    r#"use std::sync::Arc;
use warp::Filter;
use application::{snake}::{{
  {pascal}Service, GetAll{pascal}QueryHandler, GetAll{pascal}QueryHandlerTrait,
}};
use crate::errors::ApiError;

pub fn route<T>(
  handler: Arc<GetAll{pascal}QueryHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where T: {pascal}Service + Send + Sync + 'static,
{{
  warp::path!("{snake}")
    .and(warp::get())
    .and(warp::any().map(move || handler.clone()))
    .and_then(|handler: Arc<GetAll{pascal}QueryHandler<T>>| async move {{
      match handler.handle().await {{
        Ok(items) => Ok(warp::reply::json(&items)),
        Err(e) => Err(warp::reject::custom(ApiError::from(e))),
      }}
    }})
}}
"#,
    pascal = pascal,
    snake = snake
  );

  let get_by_id_endpoint = format!(
    r#"use crate::errors::ApiError;
use std::sync::Arc;
use warp::Filter;
use application::{snake}::{{
  {pascal}Service, GetById{pascal}QueryHandler, GetById{pascal}QueryHandlerTrait,
}};
use domain::queries::GetById{pascal}Query;

pub fn route<T>(
  handler: Arc<GetById{pascal}QueryHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  warp::path!("{snake}" / i32)
    .and(warp::get())
    .and(with_handler(handler))
    .and_then(handle_get_by_id_{snake})
}}

fn with_handler<T>(
  handler: Arc<GetById{pascal}QueryHandler<T>>,
) -> impl Filter<
  Extract = (Arc<GetById{pascal}QueryHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  warp::any().map(move || handler.clone())
}}

async fn handle_get_by_id_{snake}<T>(
  id: i32,
  handler: Arc<GetById{pascal}QueryHandler<T>>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  match handler.handle(GetById{pascal}Query {{ id }}).await {{
    Ok(item) => match item {{
      Some(item) => Ok(warp::reply::json(&item)),
      None => Err(warp::reject::custom(ApiError::NotFound(format!(
        "{pascal} {{}} not found",
        id
      )))),
    }},
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }}
}}
"#,
    pascal = pascal,
    snake = snake
  );

  let list_endpoint = format!(
    r#"use std::sync::Arc;
use warp::Filter;
use application::{snake}::{{
  {pascal}Service, List{pascal}QueryHandler, List{pascal}QueryHandlerTrait,
}};
use crate::errors::ApiError;
use domain::queries::List{pascal};

pub fn route<T>(
  handler: Arc<List{pascal}QueryHandler<T>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  warp::path!("{snake}" / "list")
    .and(warp::get())
    .and(warp::query::<List{pascal}>())
    .and(with_handler(handler))
    .and_then(handle_list_{snake})
}}

fn with_handler<T>(
  handler: Arc<List{pascal}QueryHandler<T>>,
) -> impl Filter<
  Extract = (Arc<List{pascal}QueryHandler<T>>,),
  Error = std::convert::Infallible,
> + Clone
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  warp::any().map(move || handler.clone())
}}

async fn handle_list_{snake}<T>(
  query: List{pascal},
  handler: Arc<List{pascal}QueryHandler<T>>,
) -> Result<impl warp::Reply, warp::Rejection>
where
  T: {pascal}Service + Send + Sync + 'static,
{{
  match handler.handle(query).await {{
    Ok(items) => Ok(warp::reply::json(&items)),
    Err(e) => Err(warp::reject::custom(ApiError::from(e))),
  }}
}}
"#,
    pascal = pascal,
    snake = snake
  );

  let mod_rs = format!(
    r#"mod create;
mod update;
mod delete;
mod get_all;
mod get_by_id;
mod list;

use std::sync::Arc;
use warp::Filter;
use application::{snake}::{{
  Create{pascal}CommandHandler, Update{pascal}CommandHandler, Delete{pascal}CommandHandler,
  GetAll{pascal}QueryHandler, GetById{pascal}QueryHandler, List{pascal}QueryHandler,
}};
use infrastructure::{{
  repositories::{snake}_repository::{pascal}Repository,
  services::{snake}_service::{pascal}ServiceImpl,
}};
use crate::startup::AppState;

pub fn register(state: Arc<AppState>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {{
  let repo = {pascal}Repository::new(state.db.connection.clone());
  let service = Arc::new({pascal}ServiceImpl::new(repo));
  let create_handler = Arc::new(Create{pascal}CommandHandler::new(service.clone()));
  let update_handler = Arc::new(Update{pascal}CommandHandler::new(service.clone()));
  let delete_handler = Arc::new(Delete{pascal}CommandHandler::new(service.clone()));
  let get_all_handler = Arc::new(GetAll{pascal}QueryHandler::new(service.clone()));
  let get_by_id_handler = Arc::new(GetById{pascal}QueryHandler::new(service.clone()));
  let list_handler = Arc::new(List{pascal}QueryHandler::new(service.clone()));

  create::route(create_handler)
    .or(update::route(update_handler))
    .or(delete::route(delete_handler))
    .or(get_all::route(get_all_handler))
    .or(get_by_id::route(get_by_id_handler))
    .or(list::route(list_handler))
}}
"#,
    pascal = pascal,
    snake = snake
  );

  let endpoint_dir = output_dir
    .join("packages/api/src/endpoints")
    .join(&snake);
  fs::create_dir_all(&endpoint_dir).unwrap();

  fs::write(endpoint_dir.join("create.rs"), create_endpoint).unwrap();
  fs::write(endpoint_dir.join("update.rs"), update_endpoint).unwrap();
  fs::write(endpoint_dir.join("delete.rs"), delete_endpoint).unwrap();
  fs::write(endpoint_dir.join("get_all.rs"), get_all_endpoint).unwrap();
  fs::write(endpoint_dir.join("get_by_id.rs"), get_by_id_endpoint).unwrap();
  fs::write(endpoint_dir.join("list.rs"), list_endpoint).unwrap();
  fs::write(endpoint_dir.join("mod.rs"), mod_rs).unwrap();

  for f in &["create.rs", "update.rs", "delete.rs", "get_all.rs", "get_by_id.rs", "list.rs", "mod.rs"] {
    println!(
      "  [api] packages/api/src/endpoints/{snake}/{f}",
      snake = snake,
      f = f
    );
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn generates_api_endpoint_files() {
    let tmp = env::temp_dir().join("codegen_test_api");
    generate("member", &[], &tmp);
    let mod_content = fs::read_to_string(
      tmp.join("packages/api/src/endpoints/member/mod.rs"),
    )
    .unwrap();
    assert!(mod_content.contains("mod create"));
    let create_content = fs::read_to_string(
      tmp.join("packages/api/src/endpoints/member/create.rs"),
    )
    .unwrap();
    assert!(create_content.contains("Create{pascal}Command") || create_content.contains("CreateMemberCommand"));
    fs::remove_dir_all(&tmp).ok();
  }

  #[test]
  fn generates_all_seven_endpoint_files() {
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
    let tmp = std::env::temp_dir().join("codegen_v2_api_update");
    generate("post", &[], &tmp);
    let content = std::fs::read_to_string(
      tmp.join("packages/api/src/endpoints/post/update.rs")
    ).unwrap();
    assert!(content.contains("warp::path!(\"post\" / i32)"), "update must use path param");
    assert!(content.contains("cmd.id = post_id"), "update must inject id into cmd");
    std::fs::remove_dir_all(&tmp).ok();
  }
}
