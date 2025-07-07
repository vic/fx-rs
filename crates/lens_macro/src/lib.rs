use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Lens)]
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
                ::fx::Lens::new()
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

#[proc_macro_derive(HasPut)]
pub fn derive_has_put(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("HasPut can only be derived for structs with named fields"),
        },
        _ => panic!("HasPut can only be derived for structs"),
    };

    let has_impls = fields.iter().map(|f| {
        let fname = f.ident.as_ref().unwrap();
        let fty = &f.ty;
        quote! {
            impl #impl_generics ::fx::Has<#fty> for #name #ty_generics #where_clause {
                fn get(&self) -> &#fty {
                    &self.#fname
                }
            }
        }
    });
    let put_impls = fields.iter().map(|f| {
        let fname = f.ident.as_ref().unwrap();
        let fty = &f.ty;
        quote! {
            impl #impl_generics ::fx::Put<#fty> for #name #ty_generics #where_clause {
                fn put(mut self, value: #fty) -> Self {
                    self.#fname = value;
                    self
                }
            }
        }
    });

    let expanded = quote! {
        #(#has_impls)*
        #(#put_impls)*
    };
    TokenStream::from(expanded)
}
