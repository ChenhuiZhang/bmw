mod context;

use context::PowerSupplyContext;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Gauge)]
pub fn derive_gauge(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    PowerSupplyContext::new(input, "GaugeBase")
        .generate()
        .into()
}

#[proc_macro_derive(GaugeAdv)]
pub fn derive_gauge_advance(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    PowerSupplyContext::new(input, "GaugeAdvance")
        .generate()
        .into()
}

#[proc_macro_derive(Charger)]
pub fn derive_charger(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    PowerSupplyContext::new(input, "ChargerBase")
        .generate()
        .into()
}
