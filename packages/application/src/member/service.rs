use async_trait::async_trait;
use common::{AppError, Paged};
use domain::aggregate::member::Member;

#[async_trait]
pub trait MemberService: Send + Sync {
  async fn get_all(&self) -> Result<Vec<Member>, AppError>;
  async fn get_by_id(&self, id: i32) -> Result<Option<Member>, AppError>;
  async fn list(&self, page: u64, page_size: u64) -> Result<Paged<Member>, AppError>;
  async fn create(&self, item: Member) -> Result<Member, AppError>;
  async fn update(&self, item: Member) -> Result<Member, AppError>;
  async fn delete(&self, id: i32) -> Result<Option<i32>, AppError>;
}
