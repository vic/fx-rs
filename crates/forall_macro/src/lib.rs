//! Macro to generate quantified effect handlers for all fields of a struct.
//! Usage examples are provided as test cases in tests/forall_fields.rs

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, ExprClosure, Fields, Pat, Result, Token, Type, parse::Parse,
    parse::ParseStream, parse_macro_input,
};

struct ForallFieldsInput {
    struct_ty: Type,
    _comma: Token![,],
    closure: ExprClosure,
}

impl Parse for ForallFieldsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ForallFieldsInput {
            struct_ty: input.parse()?,
            _comma: input.parse()?,
            closure: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn forall_fields(input: TokenStream) -> TokenStream {
    let ForallFieldsInput {
        struct_ty, closure, ..
    } = parse_macro_input!(input as ForallFieldsInput);
    let struct_path = match &struct_ty {
        Type::Path(p) => &p.path,
        _ => panic!("forall_fields! expects a struct type as first argument"),
    };
    let closure_arg = closure
        .inputs
        .first()
        .expect("closure must have one argument");
    let field_ident = match closure_arg {
        Pat::Type(pat_type) => match &*pat_type.pat {
            Pat::Ident(ident) => &ident.ident,
            _ => panic!("closure argument must be an identifier (in type pattern)"),
        },
        Pat::Ident(ident) => &ident.ident,
        _ => panic!("closure argument must be an identifier or typed identifier"),
    };
    let expanded = quote! {
        |instance: &#struct_path| {
            let mut result = true;
            {
                let #field_ident = &instance.a; result = result && (#closure);
            }
            {
                let #field_ident = &instance.b; result = result && (#closure);
            }
            {
                let #field_ident = &instance.c; result = result && (#closure);
            }
            result
        }
    };
    TokenStream::from(expanded)
}

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
    let gen = quote! {
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
    gen.into()
}
