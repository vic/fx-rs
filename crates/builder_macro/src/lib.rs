extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(ContextBuilder)]
pub fn context_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let builder_name = format_ident!("{}Builder", struct_name);
    let mut field_idents = Vec::new();
    let mut field_types = Vec::new();
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in &fields_named.named {
                field_idents.push(field.ident.as_ref().unwrap());
                field_types.push(&field.ty);
            }
        }
    }
    // Marker types for each field (UpperCamelCase)
    let marker_types: Vec<_> = field_idents
        .iter()
        .map(|id| {
            let name = id.to_string();
            let mut chars = name.chars();
            let first = chars.next().unwrap().to_uppercase().collect::<String>();
            let rest = chars.collect::<String>();
            format_ident!("{}State", format!("{}{}", first, rest))
        })
        .collect();
    // Derive Clone for marker types
    let marker_structs = marker_types
        .iter()
        .map(|ty| quote! { #[derive(Clone)] pub struct #ty; });
    let absent_generics = vec![quote! { builder_types::Absent }; marker_types.len()];
    let present_generics = vec![quote! { builder_types::Present }; marker_types.len()];

    // Generate unique builder field names
    let builder_field_idents: Vec<_> = field_idents
        .iter()
        .map(|id| format_ident!("maybe_{}", id))
        .collect();

    // Derive Clone for all builder types
    let builder_struct = quote! {
        #[derive(Clone)]
        pub struct #builder_name<#(#marker_types),*> {
            #(#builder_field_idents: Option<#field_types>,)*
            _marker: std::marker::PhantomData<(#(#marker_types),*)>,
        }
    };

    // empty() for all Absent
    let empty_impl = quote! {
        impl #builder_name<#(#absent_generics),*> {
            pub fn empty() -> Self {
                Self {
                    #(#builder_field_idents: None,)*
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };

    // put_x for each field
    let mut put_methods = Vec::new();
    for (i, (id, ty)) in field_idents.iter().zip(field_types.iter()).enumerate() {
        let generics = marker_types.clone();
        let mut next_generics = generics.clone();
        next_generics[i] = format_ident!("Present");
        let assignments = builder_field_idents.iter().enumerate().map(|(j, f)| {
            if i == j {
                quote! { #f: Some(value) }
            } else {
                quote! { #f: self.#f }
            }
        });
        let put_method = quote! {
            impl<#(#generics),*> #builder_name<#(#generics),*> {
                pub fn #id(self, value: #ty) -> #builder_name<#(#next_generics),*> {
                    #builder_name {
                        #(#assignments,)*
                        _marker: std::marker::PhantomData,
                    }
                }
            }
        };
        put_methods.push(put_method);
    }

    // build() for all Present
    let build_impl = quote! {
        impl #builder_name<#(#present_generics),*> {
            pub fn build(self) -> #struct_name {
                #struct_name {
                    #(
                        #field_idents: self.#builder_field_idents.unwrap(),
                    )*
                }
            }
        }
    };

    // Accessor methods
    let mut accessor_methods = Vec::new();
    for idx in 0..field_idents.len() {
        let id = &field_idents[idx];
        let ty = &field_types[idx];
        let builder_id = &builder_field_idents[idx];
        let get_method_name = format_ident!("maybe_{}", id);
        let get_method = quote! {
            pub fn #get_method_name(&self) -> Option<#ty> {
                self.#builder_id.as_ref().map(|v| v.clone())
            }
        };
        let has_method_name = format_ident!("has_builder_{}", id);
        let has_method = quote! {
            pub fn #has_method_name(&self) -> bool {
                self.#builder_id.is_some()
            }
        };
        accessor_methods.push(get_method);
        accessor_methods.push(has_method);
    }

    // Conditional Has impls for builder
    let mut has_impls = Vec::new();
    for (i, ty) in field_types.iter().enumerate() {
        let mut generics = marker_types
            .iter()
            .map(|_| quote! { builder_types::Absent })
            .collect::<Vec<_>>();
        generics[i] = quote! { builder_types::Present };
        let builder_ty = quote! { #builder_name<#(#generics),*> };
        let builder_field = &builder_field_idents[i];
        has_impls.push(quote! {
            impl Has<#ty> for #builder_ty {
                fn get(self) -> #ty {
                    self.#builder_field.expect("Field must be present")
                }
            }
        });
    }

    // Conditional Put impls for builder (using new trait signature)
    let mut put_impls = Vec::new();
    for (i, (ty, _)) in field_types.iter().zip(field_idents.iter()).enumerate() {
        let generics = marker_types
            .iter()
            .map(|_| quote! { builder_types::Absent })
            .collect::<Vec<_>>();
        let mut next_generics = generics.clone();
        next_generics[i] = quote! { builder_types::Present };
        let builder_ty = quote! { #builder_name<#(#generics),*> };
        let next_builder_ty = quote! { #builder_name<#(#next_generics),*> };
        let builder_field = &builder_field_idents[i];
        let assignments = builder_field_idents.iter().enumerate().map(|(j, f)| {
            if i == j {
                quote! { #f: Some(value) }
            } else {
                quote! { #f: self.#f }
            }
        });
        put_impls.push(quote! {
            impl Put<#ty, #next_builder_ty> for #builder_ty {
                fn put(self, value: #ty) -> #next_builder_ty {
                    #builder_name {
                        #(#assignments,)*
                        _marker: std::marker::PhantomData,
                    }
                }
            }
        });
    }

    let expanded = quote! {
        #(#marker_structs)*
        #builder_struct
        impl<#(#marker_types),*> #builder_name<#(#marker_types),*> {
            #(#accessor_methods)*
        }
        #empty_impl
        #(#put_methods)*
        #build_impl
        #(#has_impls)*
        #(#put_impls)*
    };
    TokenStream::from(expanded)
}
