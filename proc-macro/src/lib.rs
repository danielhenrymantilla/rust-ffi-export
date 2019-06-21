extern crate proc_macro; use ::proc_macro::TokenStream;
use ::syn::{self,
    ItemFn,
    NestedMeta,
    parse_macro_input,
    parse_quote,
    spanned::Spanned,
};
use ::quote::{
    quote_spanned,
};

const IDENT_SUFFIX: &'static str = "__hack__";

#[proc_macro_attribute] pub
fn ffi_export (args: TokenStream, input: TokenStream) -> TokenStream
{
    let mut item_fn = parse_macro_input!(input as ItemFn);
    item_fn.attrs.push(parse_quote!( #[no_mangle] ));
    item_fn.vis = parse_quote!( pub );
    item_fn.unsafety = parse_quote!( unsafe );
    item_fn.abi = Some(syn::Abi {
        extern_token: Default::default(),
        name: match syn::parse(args) { // #[ffi_export(X?)]
            // X = "abi" => extern "abi"
            | Ok(NestedMeta::Literal(syn::Lit::Str(abi))) => Some(abi),
            // else =>      extern
            | Err(_) => None,
            // err
            | Ok(otherwise) => return TokenStream::from(::quote::quote_spanned! {
                otherwise.span() => compile_error!(
                    r#"Bad argument, expected #[ffi_export("...")] for an extern "..." declaration"#
                );
            }),
        },
    });
    let ident = syn::Ident::new(
        &format!("{}{}", item_fn.ident, IDENT_SUFFIX),
        item_fn.ident.span(),
    );
    let block = *item_fn.block;
    *item_fn.block = parse_quote! {
        {
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[derive(ffi_export::named_hack)]
            enum #ident {}

            let guard = ffi_export::AbortOnDrop;
            let ret = (move || #block)();
            ::core::mem::forget(guard);
            ret
        }
    };
    TokenStream::from(quote_spanned! { item_fn.span() =>
        #item_fn
    })
}

#[doc(hidden)]
#[proc_macro_derive(named_hack)] pub
fn hack (input: TokenStream) -> TokenStream
{
    let input: syn::DeriveInput = parse_macro_input!(input);
    let ident = input.ident.to_string();
    let ident = &ident[.. ident.len() - IDENT_SUFFIX.len()];
    let fname = syn::LitStr::new(ident, input.ident.span());
    TokenStream::from(quote_spanned! { input.span() =>
        macro_rules! function_name {() => (
            #fname
        )}
    })
}
