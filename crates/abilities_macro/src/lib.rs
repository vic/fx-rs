use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, ItemStruct, Token, Type,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
};

struct AbilityMethod {
    method_name: Ident,
    arg_ty: Type,
    ret_ty: Type,
}

struct AbilitiesAttrArgs {
    methods: Vec<AbilityMethod>,
}

impl Parse for AbilitiesAttrArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut methods = Vec::new();
        while !input.is_empty() {
            let method_name: Ident = input.parse()?;
            let args;
            syn::parenthesized!(args in input);
            let arg_ty: Type = args.parse()?;
            let _: Token![->] = input.parse()?;
            let ret_ty: Type = input.parse()?;
            let _ = input.parse::<Token![,]>();
            methods.push(AbilityMethod {
                method_name,
                arg_ty,
                ret_ty,
            });
        }
        Ok(AbilitiesAttrArgs { methods })
    }
}

#[proc_macro_attribute]
pub fn abilities(attr: TokenStream, item: TokenStream) -> TokenStream {
    // let args: AbilitiesAttrArgs = parse_macro_input!(attr as AbilitiesAttrArgs);
    let item_struct: ItemStruct = parse_macro_input!(item as ItemStruct);

    let expanded = quote! {
        #item_struct
    };
    expanded.into()

    // let struct_ident = &item_struct.ident;
    // let vis = &item_struct.vis;

    // let input_enum_ident = syn::Ident::new(&format!("{}Input", struct_ident), struct_ident.span());
    // let result_enum_ident = syn::Ident::new(&format!("{}Result", struct_ident), struct_ident.span());
    // let handler_trait_ident = syn::Ident::new(&format!("{}Handler", struct_ident), struct_ident.span());

    // let input_variants = methods.iter().map(|m| {
    //     let AbilityMethod { method_name, arg_ty, .. } = m;
    //     let variant = syn::Ident::new(&capitalize(&method_name.to_string()), method_name.span());
    //     quote! { #variant(#arg_ty) }
    // });

    // let result_variants = methods.iter().map(|m| {
    //     let AbilityMethod { method_name, ret_ty, .. } = m;
    //     let variant = syn::Ident::new(&capitalize(&method_name.to_string()), method_name.span());
    //     quote! { #variant(#ret_ty) }
    // });

    // let handler_methods = methods.iter().map(|m| {
    //     let AbilityMethod { method_name, arg_ty, ret_ty, .. } = m;
    //     quote! {
    //         fn #method_name<S: Clone>(input: #arg_ty) -> Fx<'f, S, #ret_ty>;
    //     }
    // });

    // let static_methods = methods.iter().map(|m| {
    //     let AbilityMethod { method_name, arg_ty, ret_ty, .. } = m;
    //     let variant = syn::Ident::new(&capitalize(&method_name.to_string()), method_name.span());
    //     quote! {
    //         pub fn #method_name<'f, S: Clone>(input: #arg_ty) -> Fx<'f, (#ability_enum_ident, S), #ret_ty> {
    //             Ability::<#ability_enum_ident, _, _>::request(#ability_enum_ident::#variant(input))
    //         }
    //     }
    // });

    // let handler_fn = quote! {
    //     pub fn handler<'f, S: Clone>(h: Box<dyn #handler_trait_ident + 'f>) -> Handler<'f, (#ability_enum_ident, S), S, #result_enum_ident, #result_enum_ident> {
    //         Handler::new(move |fx: Fx<'f, (#ability_enum_ident, S), #result_enum_ident>| {
    //             unimplemented!("handler stub");
    //         })
    //     }
    // };

    // let expanded = quote! {
    //     #vis struct #struct_ident;

    //     #[derive(Debug, Clone, PartialEq)]
    //     #vis enum #input_enum_ident {
    //         #(#input_variants),*
    //     }

    //     #[derive(Debug, Clone, PartialEq)]
    //     #vis enum #result_enum_ident {
    //         #(#result_variants),*
    //     }

    //     #vis trait #handler_trait_ident<'f> where Self: Sized {
    //         #(#handler_methods)*
    //     }

    //     impl #struct_ident {
    //         #(#static_methods)*
    //         #handler_fn
    //     }
    // };
    // expanded.into()
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
