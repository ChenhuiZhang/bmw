extern crate bmw;

use bmw::charger;
use bmw::gauge;
use bmw::log;
use bmw::BM;

fn main() {
    BM::new(Box::new(gauge::BQ27z561), Box::new(charger::BQ24296))
        .add_plugin(log::PluginLog)
        .run();
}
