use crate::charger::Charger;
use crate::gauge::Gauge;
use std::any::Any;

#[derive(Copy, Clone, Debug)]
pub enum EventType {
    UDev,
    Timer,
}

pub trait Plugin: Any + Send + Sync {
    /// For plugin init
    fn build(&self);
    /// Configures a name for the [`Plugin`]. Primarily for debugging.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    /// Plugin process with the gauge and charger info
    fn process(&self, t: EventType, gauge: &dyn Gauge, charger: &dyn Charger);
}
