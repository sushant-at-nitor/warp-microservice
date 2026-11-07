use std::{fs, path::Path};

use crate::helpers::{to_pascal_case, to_snake_case};

pub fn generate(entity: &str, output_dir: &Path) {
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

  let mod_rs = format!(
    r#"mod create;
mod get_all;
// TODO: add update, delete, get_by_id, list modules

use std::sync::Arc;
use warp::Filter;
use application::{snake}::{{
  Create{pascal}CommandHandler, GetAll{pascal}QueryHandler,
}};
use crate::startup::AppState;

pub fn register(
  _state: Arc<AppState>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {{
  // TODO: instantiate handlers and register routes
  warp::path!("{snake}" / "placeholder")
    .and(warp::get())
    .map(|| warp::reply::json(&serde_json::json!({{}})))
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
  fs::write(endpoint_dir.join("get_all.rs"), get_all_endpoint).unwrap();
  fs::write(endpoint_dir.join("mod.rs"), mod_rs).unwrap();

  println!(
    "  [api] packages/api/src/endpoints/{snake}/create.rs",
    snake = snake
  );
  println!(
    "  [api] packages/api/src/endpoints/{snake}/get_all.rs",
    snake = snake
  );
  println!(
    "  [api] packages/api/src/endpoints/{snake}/mod.rs",
    snake = snake
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn generates_api_endpoint_files() {
    let tmp = env::temp_dir().join("codegen_test_api");
    generate("member", &tmp);
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
}
