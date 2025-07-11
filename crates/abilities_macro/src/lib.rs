// Remove duplicate imports and fix macro import
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, Item, ReturnType, TraitItem, parse_macro_input};

#[proc_macro]
pub fn abilities(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::File);
    let mut output = quote! {};
    for item in input.items {
        if let Item::Trait(trait_item) = item {
            let trait_ident = &trait_item.ident;
            output.extend(quote! {
                #[derive(Clone)]
                pub struct #trait_ident;
            });
            for trait_item in &trait_item.items {
                if let TraitItem::Fn(method) = trait_item {
                    let method_ident = &method.sig.ident;
                    let ability_fn_ident =
                        syn::Ident::new(&format!("{}_ability", method_ident), method_ident.span());
                    let (ctx_ty, input_ty, ret_ty) = match extract_types(&method.sig) {
                        Some(t) => t,
                        None => continue,
                    };
                    // Associated function for ability construction
                    output.extend(quote! {
                        impl #trait_ident {
                            pub fn #ability_fn_ident<'f, F>(f: F) -> impl Ability<'f, #input_ty, #ctx_ty, #ret_ty> + 'f
                            where F: FnOnce(#input_ty) -> Fx<'f, #ctx_ty, #ret_ty> + Clone + 'f {
                                f
                            }
                            pub fn #method_ident<'f, P, A>(arg: #input_ty) -> Fx<'f, P, #ret_ty>
                            where A: Ability<'f, #input_ty, #ctx_ty, #ret_ty> + 'f + Clone, P: Pair<A, #ctx_ty> {
                                Abilities::request::<P, A>(arg)
                            }
                        }
                    });
                }
            }
        }
    }
    output.into()
}

fn extract_types(
    sig: &syn::Signature,
) -> Option<(
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
)> {
    let mut args = sig.inputs.iter();
    let ret_ty = match &sig.output {
        ReturnType::Type(_, ty) => quote! { #ty },
        ReturnType::Default => quote! { () },
    };
    match (args.next(), args.next()) {
        (Some(FnArg::Typed(arg1)), Some(FnArg::Typed(arg2))) => {
            let ctx = &arg1.ty;
            let input = &arg2.ty;
            Some((quote! { #ctx }, quote! { #input }, ret_ty))
        }
        (Some(FnArg::Typed(arg)), None) => {
            let input = &arg.ty;
            Some((quote! { () }, quote! { #input }, ret_ty))
        }
        _ => None,
    }
}
