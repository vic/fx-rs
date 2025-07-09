use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(HasFields)]
pub fn derive_has_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("HasFields derive macro only supports named structs"),
    };

    let mut impls = Vec::new();
    for field in fields {
        let field_name = match &field.ident {
            Some(ident) => ident,
            None => panic!("HasFields derive macro does not support unnamed or tuple fields"),
        };
        let field_ty = &field.ty;
        impls.push(quote! {
            impl #impl_generics ::fx::Has<#field_ty> for #name #ty_generics #where_clause {
                fn get(&self) -> &#field_ty {
                    &self.#field_name
                }
            }
        });
    }
    TokenStream::from(quote! { #(#impls)* })
}
