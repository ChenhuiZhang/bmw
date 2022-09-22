use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

pub struct PowerSupplyContext {
    name: Ident,
    path: String,
    trait_name: String,
}

impl PowerSupplyContext {
    pub fn new(input: DeriveInput, trait_name: &str) -> Self {
        let name = input.ident;

        let path = format!("{}", name.to_string().to_ascii_lowercase());

        let trait_name = format!("{}", trait_name);

        Self {
            name,
            path,
            trait_name,
        }
    }

    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let path = self.path.as_str();
        let trait_name = Ident::new(self.trait_name.as_str(), name.span());
        quote! {
            impl #trait_name for #name {
                fn path(&self) -> &'static str {
                    #path
                }
            }
        }
    }

    pub fn generate_adv(&self) -> TokenStream {
        let name = &self.name;
        let path = self.path.as_str();
        let trait_name = Ident::new(self.trait_name.as_str(), name.span());
        let trait_base = Ident::new(
            self.trait_name.replace("Advance", "Base").as_str(),
            name.span(),
        );
        quote! {
            impl #trait_base for #name {
                fn path(&self) -> &'static str {
                    #path
                }
            }

            impl #trait_name for #name {
            }
        }
    }
}
