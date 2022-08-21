use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, AttributeArgs, NestedMeta, Error, Lit::Str, Lit::Bool};
use syn::spanned::Spanned;


fn get_return_type(input: &NestedMeta) -> TokenStream {
    match input {
        NestedMeta::Meta(m) => {
            match m.path().get_ident() {
                Some(ident) => {
                    quote! {
                        #ident
                    }
                }
                None => Error::new(input.span(), "Third argument on Kraken macro should be return type").to_compile_error()
            }
        }
        NestedMeta::Lit(_) => Error::new(input.span(), "Third argument on Kraken macro should be return type (it's lit)").to_compile_error()
    }
}

fn get_http_type(input: &NestedMeta) -> TokenStream {
    match input {
        NestedMeta::Meta(m) => {
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
                None => Error::new(input.span(), "First argument on Kraken macro should be HTTP type").to_compile_error()
            }
        }
        NestedMeta::Lit(_) => Error::new(input.span(), "First argument on Kraken macro should be HTTP type").to_compile_error()
    }

}

fn get_path(input: &NestedMeta) -> TokenStream {
    let t = match input {
        NestedMeta::Meta(_) => return Error::new(input.span(), "Second argument on Kraken macro should be the path").to_compile_error(),
        NestedMeta::Lit(l) => {
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
        NestedMeta::Meta(m) => {
            if m.path().is_ident("auth") || m.path().is_ident("authenticated") || m.path().is_ident("AUTH") || m.path().is_ident("AUTHENTICATED") {
                return quote! {
                        fn authenticated_request() -> bool {
                            true
                        }
                    };
            }
        }
        NestedMeta::Lit(l) => {
            return if let Bool(lit_bool) = l {
                let do_auth:bool = lit_bool.value();
                if do_auth {
                    quote! {
                        fn authenticated_request() -> bool {
                            true
                        }
                    }
                } else {
                    quote! {
                        fn authenticated_request() -> bool {
                            false
                        }
                    }
                }

            } else {
                Error::new(input.span(), "Wrong type for path. Expecting string").to_compile_error()
            }
        }
    }
    Error::new(input.span(), "First argument on Kraken macro should be HTTP type").to_compile_error()
}


#[proc_macro_attribute]
pub fn kraken(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item_c = item.clone();
    let input = parse_macro_input!( item_c as proc_macro2::TokenStream);
    let DeriveInput { ident, .. } = parse_macro_input!(item);
    let args = parse_macro_input!(args as AttributeArgs);

    if args.len() < 3 || args.len() > 4 {
        return proc_macro::TokenStream::from(Error::new(input.span(), format!("Incorrect number of arguments[{}]. Expecting Kraken(HTTP_TYPE, URL, RETURN_TYPE, [auth])", args.len())).to_compile_error())
    }

    let http_type = get_http_type(&args[0]);
    let path = get_path(&args[1]);
    let return_type = get_return_type(&args[2]);
    let mut auth = quote! {
                        fn authenticated_request() -> bool {
                            false
                        }
                    };
    if args.len() == 4 {
        auth = process_auth(&args[3])
    }

    let x = quote! {
        #input

        impl crate::kraken::KrakenType for #ident {
                #http_type
                #path
                #auth
            }
        #[async_trait]
        impl crate::kraken::KrakenRequest for #ident {
            type R = #return_type;
        }
    };

    proc_macro::TokenStream::from(x)
}