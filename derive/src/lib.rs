//! This crate provides three derive macros.
//!
//! ```edition2021
//! # use bmw::{Charger, Gauge, GaugeAdv};
//! #
//! #[derive(Gauge)]
//! # struct BQ27621;
//!
//! #[derive(GaugeAdv)]
//! # struct BQ27z561;
//!
//! #[derive(Charger)]
//! # struct BQ24296;
//!
//! # fn main() {}
//! ```
//!

mod context;

use context::PowerSupplyContext;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Gauge)]
pub fn derive_gauge(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    PowerSupplyContext::new(input).generate_gauge_base().into()
}

#[proc_macro_derive(GaugeAdv)]
pub fn derive_gauge_advance(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    PowerSupplyContext::new(input)
        .generate_gauge_advance()
        .into()
}

#[proc_macro_derive(Charger)]
pub fn derive_charger(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    PowerSupplyContext::new(input)
        .generate_charger_base()
        .into()
}
