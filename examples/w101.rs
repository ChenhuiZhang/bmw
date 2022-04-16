extern crate bmw;

use bmw::bmw::BM;
use bmw::charger::ChargerBase;
use bmw::gauge::GaugeBase;
use bmw::log;
use bmw::Charger;
use bmw::Gauge;

#[derive(Gauge, Default)]
struct BQ27z561;

#[derive(Charger, Default)]
struct BQ24296;

fn main() {
    BM::new(Box::new(BQ27z561::default()), Box::new(BQ24296::default()))
        .add_plugin(log::PluginLog)
        .run();
}
