use std::sync::Arc;
use async_trait::async_trait;
use common::AppError;
use domain::{aggregate::member::Member, commands::UpdateMemberCommand};
use crate::member::MemberService;

#[async_trait]
pub trait UpdateMemberCommandHandlerTrait: Send + Sync {
  async fn handle(&self, cmd: UpdateMemberCommand) -> Result<Member, AppError>;
}

pub struct UpdateMemberCommandHandler<T: MemberService> {
  service: Arc<T>,
}

impl<T: MemberService> UpdateMemberCommandHandler<T> {
  pub fn new(service: Arc<T>) -> Self { Self { service } }
}

#[async_trait]
impl<T: MemberService> UpdateMemberCommandHandlerTrait for UpdateMemberCommandHandler<T> {
  async fn handle(&self, cmd: UpdateMemberCommand) -> Result<Member, AppError> {
    let mut item = self.service.get_by_id(cmd.id).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?
      .ok_or_else(|| AppError::NotFound(format!("Member {} not found", cmd.id)))?;
  if let Some(v) = cmd.name { item.name = v; }
  if let Some(v) = cmd.email { item.email = v; }
  if let Some(v) = cmd.age { item.age = Some(v); }
    self.service.update(item).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }
}
