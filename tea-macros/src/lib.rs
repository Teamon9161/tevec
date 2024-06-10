// mod match_enum;

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, FnArg, ItemFn, ReturnType};

#[allow(clippy::vec_box)]
pub(crate) fn parse_params(sig: &syn::Signature) -> Vec<Box<syn::Pat>> {
    sig.inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                Some(pat_type.pat.clone())
            } else {
                None
            }
        })
        .collect()
}

fn no_output_transform(attr: TokenStream, func: ItemFn) -> TokenStream2 {
    let attr: TokenStream2 = attr.into();
    let mut fn_sig = func.sig;
    let fn_block = func.block;

    let mut new_sig = fn_sig.clone();
    // change return type of original function
    let ori_output = new_sig.output.clone();
    let output_type = match &ori_output {
        ReturnType::Type(_, ty) => quote! { Option<#ty> },
        _ => quote! { () },
    };
    fn_sig.output = parse_quote! { -> #output_type};

    let ori_func_name = format_ident!("{}_to", fn_sig.ident);
    fn_sig.ident = ori_func_name.clone();
    // remove out parameter from new function
    new_sig.inputs = new_sig
        .inputs
        .into_iter()
        .filter(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    return pat_ident.ident != "out";
                }
            }
            true
        })
        .collect();
    let params = parse_params(&new_sig);
    quote! {
        #attr
        #fn_sig #fn_block

        #[inline]
        #attr
        #new_sig {
            self.#ori_func_name(#(#params,)* None).unwrap()
        }
    }
}

#[proc_macro_attribute]
pub fn no_out(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let out = no_output_transform(attr, input_fn);
    TokenStream::from(out)
}

#[proc_macro_derive(GetDtype)]
pub fn derive_get_data_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let data_type_impls = if let Data::Enum(data_enum) = input.data {
        data_enum.variants.into_iter().map(|variant| {
            let ident = &variant.ident;
            match ident.to_string().as_str() {
                "DateTimeS" => quote! {Self::#ident(_) => DataType::DateTime(TimeUnit::Second),},
                "DateTimeMs" => {
                    quote! {Self::#ident(_) => DataType::DateTime(TimeUnit::Millisecond),}
                }
                "DateTimeUs" => {
                    quote! {Self::#ident(_) => DataType::DateTime(TimeUnit::Microsecond),}
                }
                "DateTimeNs" => {
                    quote! {Self::#ident(_) => DataType::DateTime(TimeUnit::Nanosecond),}
                }
                _ => quote! { Self::#ident(_) => DataType::#ident,},
            }
        })
    } else {
        panic!("GetDtype can only be derived for enums");
    };

    let expanded = quote! {
        impl #impl_generics GetDtype for #name #ty_generics #where_clause {
            fn dtype(&self) -> DataType
            {
                match self {
                    #(#data_type_impls)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
