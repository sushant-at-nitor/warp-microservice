use std::sync::Arc;
use async_trait::async_trait;
use common::{AppError, Paged};
use domain::{aggregate::member::Member, queries::ListMember};
use crate::member::MemberService;

#[async_trait]
pub trait ListMemberQueryHandlerTrait: Send + Sync {
  async fn handle(&self, query: ListMember) -> Result<Paged<Member>, AppError>;
}

pub struct ListMemberQueryHandler<T: MemberService> {
  service: Arc<T>,
}

impl<T: MemberService> ListMemberQueryHandler<T> {
  pub fn new(service: Arc<T>) -> Self { Self { service } }
}

#[async_trait]
impl<T: MemberService> ListMemberQueryHandlerTrait for ListMemberQueryHandler<T> {
  async fn handle(&self, query: ListMember) -> Result<Paged<Member>, AppError> {
    let page      = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    self.service.list(page, page_size).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }
}
