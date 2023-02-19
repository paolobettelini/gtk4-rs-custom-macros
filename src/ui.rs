use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, DeriveInput, Expr, ExprLit, Ident, Lit, LitStr};

pub struct StructDefField {
    pub name: String,
    pub r#type: String,
}

pub struct StructInitField {
    pub name: String,
}

impl ToTokens for StructDefField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = syn::parse_str::<Ident>(&self.name).unwrap();
        let r#type = syn::parse_str::<Ident>(&self.r#type).unwrap();
        tokens.extend(quote! {
            #name: #r#type
        });
    }
}

impl ToTokens for StructInitField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = syn::parse_str::<Ident>(&self.name).unwrap();
        let name = &self.name;
        tokens.extend(quote! {
            #ident: builder.object(#name).unwrap()
        });
    }
}