use crate::repositories::member_repository::MemberRepository;
use application::member::MemberService;
use async_trait::async_trait;
use common::{AppError, Paged};
use domain::aggregate::member::Member;

pub struct MemberServiceImpl {
  repo: MemberRepository,
}

impl MemberServiceImpl {
  pub fn new(repo: MemberRepository) -> Self {
    Self { repo }
  }
}

#[async_trait]
impl MemberService for MemberServiceImpl {
  async fn get_all(&self) -> Result<Vec<Member>, AppError> {
    self.repo.find_all().await
      .map(|items| items.into_iter().map(Into::into).collect())
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }

  async fn get_by_id(&self, id: i32) -> Result<Option<Member>, AppError> {
    self.repo.find_by_id(id).await
      .map(|opt| opt.map(Into::into))
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }

  async fn list(&self, page: u64, page_size: u64) -> Result<Paged<Member>, AppError> {
    let p = self.repo.list_member(page, page_size).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(Paged {
      items: p.items.into_iter().map(Into::into).collect(),
      total: p.total,
      page: p.page,
      page_size: p.page_size,
      total_pages: p.total_pages,
    })
  }

  async fn create(&self, item: Member) -> Result<Member, AppError> {
    self.repo.insert(item.into()).await
      .map(Into::into)
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }

  async fn update(&self, item: Member) -> Result<Member, AppError> {
    self.repo.update(item.into()).await
      .map(Into::into)
      .map_err(|e| AppError::DatabaseError(e.to_string()))
  }

  async fn delete(&self, id: i32) -> Result<Option<i32>, AppError> {
    let result = self.repo.delete(id).await
      .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(if result.rows_affected == 0 { None } else { Some(id) })
  }
}
