use std::sync::Arc;
use async_trait::async_trait;
use common::AppError;
use domain::{aggregate::member::Member, commands::CreateMemberCommand};
use crate::member::MemberService;

#[async_trait]
pub trait CreateMemberCommandHandlerTrait: Send + Sync {
  async fn handle(&self, cmd: CreateMemberCommand) -> Result<Member, AppError>;
}

pub struct CreateMemberCommandHandler<T: MemberService> {
  service: Arc<T>,
}

impl<T: MemberService> CreateMemberCommandHandler<T> {
  pub fn new(service: Arc<T>) -> Self { Self { service } }
}

#[async_trait]
impl<T: MemberService> CreateMemberCommandHandlerTrait for CreateMemberCommandHandler<T> {
  async fn handle(&self, cmd: CreateMemberCommand) -> Result<Member, AppError> {
    let item = Member {
  id: 0,
  name: cmd.name,
  email: cmd.email,
  age: Some(cmd.age),
    };
    self.service.create(item).await.map_err(|e| AppError::DatabaseError(e.to_string()))
  }
}
