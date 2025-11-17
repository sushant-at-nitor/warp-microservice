use crate::events::DomainEvent;

pub trait Aggregate {
  type Event: DomainEvent + ?Sized;

  fn apply(&mut self, event: &Self::Event);
  fn record(&mut self, event: Self::Event);
  fn take_events(&mut self) -> Vec<Box<Self::Event>>;
}
