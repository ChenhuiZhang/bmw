use crate::plugin::Plugin;

pub struct PluginLog;

impl Plugin for PluginLog {
    fn build(&self) {}

    fn process(
        &self,
        t: crate::plugin::EventType,
        gauge: &dyn crate::gauge::GaugeBase,
        charger: &dyn crate::charger::ChargerBase,
    ) {
        println!("{}", gauge.get_capacity().unwrap());
    }
}
