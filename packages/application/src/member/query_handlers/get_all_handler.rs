use std::sync::Arc;
use async_trait::async_trait;
use common::AppError;
use domain::aggregate::member::Member;
use crate::member::MemberService;

#[async_trait]
pub trait GetAllMemberQueryHandlerTrait: Send + Sync {
  async fn handle(&self) -> Result<Vec<Member>, AppError>;
}

pub struct GetAllMemberQueryHandler<T: MemberService> {
  service: Arc<T>,
}

impl<T: MemberService> GetAllMemberQueryHandler<T> {
  pub fn new(service: Arc<T>) -> Self { Self { service } }
}

#[async_trait]
impl<T: MemberService> GetAllMemberQueryHandlerTrait for GetAllMemberQueryHandler<T> {
  async fn handle(&self) -> Result<Vec<Member>, AppError> {
    self.service.get_all().await.map_err(|e| AppError::DatabaseError(e.to_string()))
  }
}
