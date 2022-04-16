//! # Battery Manager for Wearable devices
//!
//! BMW is a framework for the wearable devices which based on gauge and charger
//! to manage the battery and also trigger actions by the plugins
//!

pub mod bmw;
pub mod charger;
pub mod gauge;
pub mod log;

mod plugin;
mod poll;

pub use bmw_derive::*;

use crate::plugin::EventType;
use charger::ChargerBase;
use gauge::GaugeBase;
