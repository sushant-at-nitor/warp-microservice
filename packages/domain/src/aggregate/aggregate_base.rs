use crate::events::DomainEvent;

pub trait Aggregate {
  type Event: DomainEvent;

  fn apply(&mut self, event: &Self::Event);
  fn record(&mut self, event: Self::Event);
  fn take_events(&mut self) -> Vec<Self::Event>;
}
