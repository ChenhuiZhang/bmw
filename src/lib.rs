//! # Battery Manager for Wearable devices
//!
//! BMW is a framework for the wearable devices which based on gauge and charger
//! to manage the battery and also trigger actions by the plugins
//!

pub mod charger;
pub mod gauge;
pub mod log;
mod plugin;
mod poll;

use crate::plugin::EventType;
use crate::plugin::Plugin;
pub use bmw_derive::*;
use charger::ChargerBase;
use gauge::GaugeBase;

use std::sync::mpsc::channel;
use std::thread;

pub struct BM {
    gauge: Box<dyn GaugeBase>,
    charger: Box<dyn ChargerBase>,
    plugins: Vec<Box<dyn Plugin>>,
}

impl BM {
    pub fn new(g: Box<dyn GaugeBase>, c: Box<dyn ChargerBase>) -> BM {
        Self {
            gauge: g,
            charger: c,
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin<T>(&mut self, plugin: T) -> &mut Self
    where
        T: Plugin,
    {
        println!("added plugin: {}", plugin.name());
        //plugin.build(self);
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn run(&self) {
        let (tx, rx) = channel::<EventType>();

        thread::spawn(move || {
            let socket = udev::MonitorBuilder::new()?
                .match_subsystem("power_supply")?
                .listen()?;
            poll::poll(socket, tx)
        });

        loop {
            let t = rx.recv().unwrap();

            for p in &self.plugins {
                p.process(t, self.gauge.as_ref(), self.charger.as_ref());
            }
        }
    }
}
