use crate::entities::member;
use domain::aggregate::member::Member;
use sea_orm::ActiveValue;

impl From<member::Model> for Member {
  fn from(model: member::Model) -> Self {
    Member {
      id: model.id,
      name: model.name,
      email: model.email,
      age: model.age,
    }
  }
}

impl From<Member> for member::ActiveModel {
  fn from(item: Member) -> Self {
    member::ActiveModel {
      id: if item.id == 0 {
        ActiveValue::not_set()
      } else {
        ActiveValue::Set(item.id)
      },
      name: ActiveValue::Set(item.name),
      email: ActiveValue::Set(item.email),
      age: ActiveValue::Set(item.age),
    }
  }
}
