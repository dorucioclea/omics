use common::event::BasicEvent;
use common::model::AggregateRoot;

pub type PlanId = String;

pub struct Plan {
    base: AggregateRoot<PlanId, BasicEvent>,
}