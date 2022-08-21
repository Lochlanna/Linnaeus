use std::process::id;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, ItemStruct, DeriveInput, Data, Fields, AttributeArgs, NestedMeta, Meta, Lit, Error, Lit::Str, Lit::Bool};
use syn::spanned::Spanned;
use darling::FromDeriveInput;
use http;

// pub trait KrakenType {
//     fn kraken_path() -> &'static str;
//     fn http_type() -> http::Method;
//     fn authenticated_request() -> bool;
// }
fn get_http_type(input: &NestedMeta) -> TokenStream {
    let t = match input {
        syn::NestedMeta::Meta(m) => {
            match m.path().get_ident() {
                Some(ident) => {
                    //TODO can we actually validate it here?
                    let ident_str = ident.to_string();
                    let ident = Ident::new(ident_str.to_uppercase().as_str(), ident.span());
                    quote! {
                        fn http_type() -> http::Method {
                            http::Method::#ident
                        }
                    }
                }
                None => return Error::new(input.span(), "First argument on Kraken macro should be HTTP type").to_compile_error()
            }
        }
        syn::NestedMeta::Lit(l) => return Error::new(input.span(), "First argument on Kraken macro should be HTTP type").to_compile_error()
    };
    t
}

fn get_path(input: &NestedMeta) -> TokenStream {
    let t = match input {
        syn::NestedMeta::Meta(m) => return Error::new(input.span(), "Second argument on Kraken macro should be the path").to_compile_error(),
        syn::NestedMeta::Lit(l) => {
            if let Str(lit_str) = l {
                let url = lit_str.value();
                quote! {
                        fn kraken_path() -> &'static str {
                            #url
                        }
                    }
            } else {
                return Error::new(input.span(), "Wrong type for path. Expecting string").to_compile_error()
            }
        }
    };
    t
}

fn process_auth(input: &NestedMeta) -> TokenStream {
    match input {
        syn::NestedMeta::Meta(m) => {
            if m.path().is_ident("auth") || m.path().is_ident("authenticated") || m.path().is_ident("AUTH") || m.path().is_ident("AUTHENTICATED") {
                return quote! {
                        fn authenticated_request() -> bool {
                            true
                        }
                    };
            }
        }
        syn::NestedMeta::Lit(l) => {
            if let Bool(lit_bool) = l {
                let do_auth:bool = lit_bool.value();
                if do_auth {
                    return quote! {
                        fn authenticated_request() -> bool {
                            true
                        }
                    };
                }
                return quote! {
                        fn authenticated_request() -> bool {
                            false
                        }
                    };
            } else {
                return Error::new(input.span(), "Wrong type for path. Expecting string").to_compile_error()
            }
        }
    }
    Error::new(input.span(), "First argument on Kraken macro should be HTTP type").to_compile_error()
}


#[proc_macro_attribute]
pub fn kraken(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as proc_macro2::TokenStream);
    let args = parse_macro_input!(args as AttributeArgs);

    if args.len() != 2 && args.len() != 3 {
        return proc_macro::TokenStream::from(Error::new(input.span(), "Incorrect number of arguments. Expecting Kraken(HTTP_TYPE, URL, [auth])").to_compile_error())
    }

    let http_type = get_http_type(&args[0]);
    let path = get_path(&args[1]);
    let mut auth = quote! {
                        fn authenticated_request() -> bool {
                            false
                        }
                    };
    if args.len() == 3 {
        auth = process_auth(&args[2])
    }

    let x = quote! {
        #input

        impl crate::kraken::KrakenType for crate::kraken::market::Time {
                #http_type
                #path
                #auth
            }
    };

    proc_macro::TokenStream::from(x)
}