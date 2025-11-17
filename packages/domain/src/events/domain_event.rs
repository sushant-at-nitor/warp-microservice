use chrono::{DateTime, Utc};
use std::any::Any;
use std::fmt::Debug;

/// Every domain event implements this.
pub trait DomainEvent: Send + Sync + Debug {
  fn as_any(&self) -> &dyn Any;
  fn event_type(&self) -> &'static str {
    std::any::type_name::<Self>()
  }
  fn occurred_on(&self) -> DateTime<Utc>;
}
