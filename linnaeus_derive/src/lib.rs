use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, ItemStruct, DeriveInput, Data, Fields};
use syn::spanned::Spanned;
use proc_macro2::{Ident, Span};
use darling::FromDeriveInput;


#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(kraken_path))]
struct Opts {
    url: Option<String>,
    auth: bool
}

fn get_fields_types(item:&DeriveInput) -> Result<(Vec<&syn::Ident>, Vec<&syn::Type> , &syn::Ident), TokenStream> {
    let struct_name = &item.ident;

    let struct_data = if let Data::Struct(struct_body) = &item.data {
        struct_body
    } else {
        return Err(syn::Error::new(item.span() ,"kraken derive macro only works on structs").to_compile_error().into());
    };

    let fields = if let Fields::Named(named_fields) = &struct_data.fields {
        named_fields
    } else {
        return Err(syn::Error::new(item.span(), "Named fields are missing").to_compile_error().into());
    };
    let mut fields_vec = Vec::new();
    let mut fields_type_vec = Vec::new();
    for field in &fields.named {
        let mut skip = false;
        for attr in &field.attrs {
            if attr.path.is_ident("kraken_skip_field") {
                skip = true;
                break;
            }
        }
        if skip {
            continue;
        }

        fields_type_vec.push(&field.ty);
        fields_vec.push(field.ident.as_ref().unwrap());
    }
    Ok((fields_vec, fields_type_vec, struct_name))
}

#[proc_macro_derive(Kraken, attributes(kraken_path, auth))]
pub fn kraken_derrive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let opts = Opts::from_derive_input(&input).expect("Wrong options");

    let (fields_vec, fields_type_vec, struct_name)  = match get_fields_types(&input) {
        Ok(sd) => sd,
        Err(err) => return err
    };

    let kraken_request = match opts.auth {
        true => quote! {
            impl crate::kraken::AuthenticatedKrakenRequest for #struct_name {}
        },
        false => quote! {
            impl crate::kraken::PublicKrakenRequest for #struct_name {}
        }
    };

    let path = match opts.url {
        Some(x) => quote! {
            impl crate::kraken::KrakenPath for #struct_name {
                fn kraken_path() -> &'static str {
                    #x
                }
            }

        },
        None => quote! {},
    };

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl crate::utils::ToBTree for #struct_name {
            fn to_b_tree(&self) -> std::collections::BTreeMap<std::string::String, crate::utils::PrimitiveValue> {
                let mut tree = std::collections::BTreeMap::new();
                #(
                    tree.insert(std::string::String::from(stringify!(#fields_vec)), crate::utils::PrimitiveValue::from(self.#fields_vec.clone()));
                )*
                tree
            }
        }
        #path
        #kraken_request
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}