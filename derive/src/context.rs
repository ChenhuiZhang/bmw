use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

pub struct PowerSupplyContext {
    name: Ident,
    path: String,
}

impl PowerSupplyContext {
    pub fn new(input: DeriveInput) -> Self {
        let name = input.ident;

        let path = format!("{}", name.to_string().to_ascii_lowercase());

        Self { name, path }
    }

    pub fn generate_gauge_base(&self) -> TokenStream {
        let name = &self.name;
        let path = self.path.as_str();
        quote! {
            impl bmw::gauge::GaugeBase for #name {
                fn path(&self) -> &'static str {
                    #path
                }
            }
        }
    }

    pub fn generate_gauge_advance(&self) -> TokenStream {
        let name = &self.name;
        let path = self.path.as_str();
        quote! {
            impl bmw::gauge::GaugeBase for #name {
                fn path(&self) -> &'static str {
                    #path
                }
            }

            impl bmw::gauge::GaugeAdvance for #name {
            }
        }
    }

    pub fn generate_charger_base(&self) -> TokenStream {
        let name = &self.name;
        let path = self.path.as_str();
        quote! {
            impl bmw::charger::ChargerBase for #name {
                fn path(&self) -> &'static str {
                    #path
                }
            }
        }
    }
}
