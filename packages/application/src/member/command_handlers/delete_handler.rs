use std::sync::Arc;
use async_trait::async_trait;
use common::AppError;
use domain::commands::DeleteMemberCommand;
use crate::member::MemberService;

#[async_trait]
pub trait DeleteMemberCommandHandlerTrait: Send + Sync {
  async fn handle(&self, cmd: DeleteMemberCommand) -> Result<i32, AppError>;
}

pub struct DeleteMemberCommandHandler<T: MemberService> {
  service: Arc<T>,
}

impl<T: MemberService> DeleteMemberCommandHandler<T> {
  pub fn new(service: Arc<T>) -> Self { Self { service } }
}

#[async_trait]
impl<T: MemberService> DeleteMemberCommandHandlerTrait for DeleteMemberCommandHandler<T> {
  async fn handle(&self, cmd: DeleteMemberCommand) -> Result<i32, AppError> {
    self.service.delete(cmd.id).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?
      .ok_or_else(|| AppError::NotFound(format!("Member {} not found", cmd.id)))
  }
}
