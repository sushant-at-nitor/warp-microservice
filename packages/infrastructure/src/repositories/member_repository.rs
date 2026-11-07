use crate::entities::member;
use common::Paged;
use sea_orm::{
  ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult,
  EntityTrait, PaginatorTrait, QuerySelect,
};

pub struct MemberRepository {
  db: DatabaseConnection,
}

impl MemberRepository {
  pub fn new(db: DatabaseConnection) -> Self {
    Self { db }
  }

  pub async fn find_all(&self) -> Result<Vec<member::Model>, DbErr> {
    member::Entity::find().all(&self.db).await
  }

  pub async fn list_member(
    &self,
    page: u64,
    page_size: u64,
  ) -> Result<Paged<member::Model>, DbErr> {
    let page = page.max(1);
    let page_size = page_size.clamp(1, 100);
    let offset = (page - 1) * page_size;
    let total = member::Entity::find().count(&self.db).await?;
    let items = member::Entity::find()
      .offset(offset)
      .limit(page_size)
      .all(&self.db)
      .await?;
    let total_pages = total.div_ceil(page_size);
    Ok(Paged { items, total, page, page_size, total_pages })
  }

  pub async fn find_by_id(&self, id: i32) -> Result<Option<member::Model>, DbErr> {
    member::Entity::find_by_id(id).one(&self.db).await
  }

  pub async fn insert(
    &self,
    model: member::ActiveModel,
  ) -> Result<member::Model, DbErr> {
    model.insert(&self.db).await
  }

  pub async fn update(
    &self,
    model: member::ActiveModel,
  ) -> Result<member::Model, DbErr> {
    model.update(&self.db).await
  }

  pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
    member::Entity::delete_by_id(id).exec(&self.db).await
  }
}
