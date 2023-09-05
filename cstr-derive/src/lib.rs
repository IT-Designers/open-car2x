#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Data, DataEnum, DataStruct, DataUnion, DeriveInput, Ident};

#[proc_macro_derive(CStr)]
pub fn derive_cstr(input_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_stream as DeriveInput);
    let name = input.ident.clone();

    match &input.data {
        Data::Struct(strct) => derive_struct(name, strct),
        Data::Enum(enm) => derive_enum(name, enm),
        Data::Union(union) => derive_union(name, union),
    }
}

fn derive_struct(name: Ident, _strct: &DataStruct) -> TokenStream {
    quote! (
        impl #name {
            fn as_cstr(&self) -> &'static std::ffi::CStr {
                cstr::cstr!(#name)
            }
        }
    )
    .into()
}

fn derive_enum(name: Ident, enm: &DataEnum) -> TokenStream {
    let variants = enm.variants.iter().map(|v| {
        let ident = &v.ident;
        if v.fields.is_empty() {
            quote!( Self::#ident => cstr::cstr!(#ident) )
        } else if v.fields.iter().any(|f| f.ident.is_some()) {
            quote!( Self::#ident { .. } => cstr::cstr!(#ident) )
        } else {
            quote!( Self::#ident ( .. ) => cstr::cstr!(#ident) )
        }
    });
    quote!(
        impl #name {
            fn as_cstr(&self) -> &'static std::ffi::CStr {
                match self {
                    #(#variants),*
                }
            }
        }
    )
    .into()
}

fn derive_union(name: Ident, _un: &DataUnion) -> TokenStream {
    quote! (
        impl #name {
            fn as_cstr(&self) -> &'static std::ffi::CStr {
                cstr::cstr!(#name)
            }
        }
    )
    .into()
}
