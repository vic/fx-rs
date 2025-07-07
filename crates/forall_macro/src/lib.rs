//! Macro to generate quantified effect handlers for all fields of a struct.
//! Usage examples are provided as test cases in tests/forall_fields.rs

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(ForallFields)]
pub fn derive_forall_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("ForallFields only supports structs with named fields"),
        },
        _ => panic!("ForallFields only supports structs"),
    };
    let field_idents: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let enum_name = format_ident!("{}Field", name);
    let enum_variants = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! { #ident(&'a #ty) }
    });
    let enum_vis = vis;
    let quoted = quote! {
        #[allow(non_camel_case_types)]
        #enum_vis enum #enum_name<'a> {
            #(#enum_variants),*
        }
        impl #name {
            pub fn forall_fields<'a, F, R>(&'a self, mut f: F) -> Vec<R>
            where
                F: FnMut(#enum_name<'a>) -> R,
            {
                vec![
                    #(f(#enum_name::#field_idents(&self.#field_idents))),*
                ]
            }
        }
    };
    quoted.into()
}
