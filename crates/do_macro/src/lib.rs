use proc_macro::TokenStream;
use quote::quote;
use syn::{Block, Stmt, parse_macro_input};

#[proc_macro]
pub fn fx_do(input: TokenStream) -> TokenStream {
    let block = parse_macro_input!(input as Block);
    let stmts = &block.stmts;
    let expanded = expand_do_block(stmts);
    TokenStream::from(expanded)
}

fn expand_do_block(stmts: &[Stmt]) -> proc_macro2::TokenStream {
    if stmts.is_empty() {
        return quote! { () };
    }
    let (first, rest) = stmts.split_first().unwrap();
    match first {
        // Handle letf! pattern: letf!(var = expr);
        Stmt::Macro(mac_stmt) => {
            let mac = &mac_stmt.mac;
            let mac_path = mac.path.segments.last().map(|s| s.ident.to_string());
            if let Some(ref ident) = mac_path {
                if ident == "letf" {
                    // Parse letf!(var = expr)
                    let tokens = &mac.tokens;
                    let tokens: proc_macro2::TokenStream = tokens.clone();
                    let parsed: syn::Stmt = syn::parse2(quote! { let #tokens; }).unwrap();
                    if let syn::Stmt::Local(local) = parsed {
                        if let Some(init) = &local.init {
                            let pat = &local.pat;
                            let expr = &init.expr;
                            let rest_expanded = expand_do_block(rest);
                            return quote! {
                                (#expr).flat_map(move |#pat| { #rest_expanded })
                            };
                        }
                    }
                } else if ident == "letm" {
                    // Parse letm!(var = expr)
                    let tokens = &mac.tokens;
                    let tokens: proc_macro2::TokenStream = tokens.clone();
                    let parsed: syn::Stmt = syn::parse2(quote! { let #tokens; }).unwrap();
                    if let syn::Stmt::Local(local) = parsed {
                        if let Some(init) = &local.init {
                            let pat = &local.pat;
                            let expr = &init.expr;
                            let rest_expanded = expand_do_block(rest);
                            return quote! {
                                (#expr).map_m(move |#pat| { #rest_expanded })
                            };
                        }
                    }
                }
            }
            // fallback to default
            let rest_expanded = expand_do_block(rest);
            quote! {
                #first
                #rest_expanded
            }
        }
        Stmt::Local(local) => {
            if let Some(init) = &local.init {
                let pat = &local.pat;
                let expr = &init.expr;
                let rest_expanded = expand_do_block(rest);
                // If the right-hand side is a .same() call, use map_m
                if let syn::Expr::MethodCall(method_call) = &**expr {
                    if method_call.method == "same" && method_call.args.is_empty() {
                        let base = &method_call.receiver;
                        return quote! {
                            (#base).map_m(move |#pat| { #rest_expanded })
                        };
                    } else if method_call.method == "bind" && method_call.args.is_empty() {
                        let base = &method_call.receiver;
                        return quote! {
                            (#base).flat_map(move |#pat| { #rest_expanded })
                        };
                    }
                }
                // Otherwise, wrap in Fx::value
                quote! {
                    (Fx::value(#expr)).map_m(move |#pat| { #rest_expanded })
                }
            } else {
                let rest_expanded = expand_do_block(rest);
                quote! {
                    let #local
                    #rest_expanded
                }
            }
        }
        Stmt::Expr(expr, _) if rest.is_empty() => {
            quote! { Fx::value(#expr) }
        }
        Stmt::Expr(expr, _) => {
            let rest_expanded = expand_do_block(rest);
            // Use flat_map for sequencing expressions (potential context change)
            quote! {
                (#expr).flat_map(move |_| { #rest_expanded })
            }
        }
        _ => {
            let rest_expanded = expand_do_block(rest);
            quote! {
                #first
                #rest_expanded
            }
        }
    }
}
