use std::any::Any;

use crate::{charger::ChargerBase, gauge::GaugeBase};

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
    fn process(&self, t: EventType, gauge: &dyn GaugeBase, charger: &dyn ChargerBase);
}
