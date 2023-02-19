extern crate proc_macro;
extern crate quote;

pub(crate) use proc_macro::TokenStream;
use quote::{quote, ToTokens};
pub(crate) use syn::{parse::Parse, parse_macro_input, Expr, Ident};

struct Properties {
    properties: Vec<Property>,
}

struct Property {
    field: Ident,
    value: Expr,
}

impl Parse for Properties {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut properties = vec![];

        while !input.is_empty() {
            let field: Ident = input.parse()?;
            let value: Expr = input.parse()?;

            properties.push(Property { field, value });
        }
        Ok(Properties { properties })
    }
}

impl ToTokens for Property {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let field = &self.field;
        let value = &self.value;
        tokens.extend(quote! {
            // .field(value)
            .#field(#value)
        });
    }
}

pub fn expand(name: Ident, input: TokenStream) -> TokenStream {
    let properties = parse_macro_input!(input as Properties);
    let inputs = properties.properties.into_iter();

    let expanded = quote! {
        #name::builder()
            #(
                // .field(value)
                #inputs
            )*
            .build()
    };

    TokenStream::from(expanded)
}

macro_rules! make_macro {
    ($name:ident, $struct_name:tt) => {
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            expand(
                Ident::new(stringify!($struct_name), Span::call_site()),
                input,
            )
        }
    };
}

pub(crate) use make_macro;
