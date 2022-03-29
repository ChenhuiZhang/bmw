use crate::charger::Charger;
use crate::gauge::Gauge;
use std::any::Any;

#[derive(Copy, Clone, Debug)]
pub enum EventType {
    UDev,
    Timer,
}

/// A collection of Bevy App logic and configuration
///
/// Plugins configure an [`App`](crate::App). When an [`App`](crate::App) registers
/// a plugin, the plugin's [`Plugin::build`] function is run.
pub trait Plugin: Any + Send + Sync {
    /// Configures the [`App`] to which this plugin is added.
    fn build(&self);
    /// Configures a name for the [`Plugin`]. Primarily for debugging.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn process(&self, t: EventType, gauge: &dyn Gauge, charger: &dyn Charger) {
        println!("Hello ");
    }
}
