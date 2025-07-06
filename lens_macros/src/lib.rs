use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Lenses)]
pub fn derive_lens(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Lens can only be derived for structs with named fields"),
        },
        _ => panic!("Lens can only be derived for structs"),
    };

    let lenses = fields.iter().map(|f| {
        let fname = f.ident.as_ref().unwrap();
        let fty = &f.ty;
        let lens_name = format_ident!("lens_{}", fname);
        quote! {
            #vis fn #lens_name<'f>() -> ::fx::Lens<'f, #name #ty_generics, #fty> {
                ::fx::Lens::new(|s: #name #ty_generics| s.#fname.clone(), |mut s, v| { s.#fname = v; s })
            }
        }
    });

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#lenses)*
        }
    };
    TokenStream::from(expanded)
}
