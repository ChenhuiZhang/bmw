extern crate bmw;

use bmw::bmw::BM;
use bmw::log;
use bmw::Charger;
use bmw::Gauge;

#[derive(Gauge, Default)]
struct BQ27621;

#[derive(Charger, Default)]
struct BQ24296;

fn main() {
    BM::new(Box::new(BQ27621::default()), Box::new(BQ24296::default()))
        .add_plugin(log::PluginLog)
        .run();
}
