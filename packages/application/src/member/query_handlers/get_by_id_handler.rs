use std::sync::Arc;
use async_trait::async_trait;
use common::AppError;
use domain::{aggregate::member::Member, queries::GetMemberByIdQuery};
use crate::member::MemberService;

#[async_trait]
pub trait GetByIdMemberQueryHandlerTrait: Send + Sync {
  async fn handle(&self, query: GetMemberByIdQuery) -> Result<Member, AppError>;
}

pub struct GetByIdMemberQueryHandler<T: MemberService> {
  service: Arc<T>,
}

impl<T: MemberService> GetByIdMemberQueryHandler<T> {
  pub fn new(service: Arc<T>) -> Self { Self { service } }
}

#[async_trait]
impl<T: MemberService> GetByIdMemberQueryHandlerTrait for GetByIdMemberQueryHandler<T> {
  async fn handle(&self, query: GetMemberByIdQuery) -> Result<Member, AppError> {
    self.service.get_by_id(query.id).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?
      .ok_or_else(|| AppError::NotFound(format!("Member {} not found", query.id)))
  }
}
