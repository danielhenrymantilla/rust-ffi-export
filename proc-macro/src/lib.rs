extern crate proc_macro; use ::proc_macro::TokenStream;
use ::syn::{
    ItemFn,
    NestedMeta,
    parse_macro_input,
    parse_quote,
};

#[proc_macro_attribute] pub
fn ffi_export (args: TokenStream, input: TokenStream) -> TokenStream
{
    let mut fn_decl = parse_macro_input!(input as ItemFn);
    fn_decl.attrs.push(parse_quote!{ #[no_mangle] });
    fn_decl.vis = ::syn::VisPublic { pub_token: Default::default() }.into();
    fn_decl.unsafety = Some(Default::default());
    fn_decl.abi = Some(::syn::Abi {
        extern_token: Default::default(),
        name: match ::syn::parse_macro_input::parse(args) {
            | Err(_) => None,
            | Ok(NestedMeta::Literal(::syn::Lit::Str(name))) => Some(name),
            | Ok(otherwise) => return TokenStream::from(::quote::quote_spanned!{
                ::syn::spanned::Spanned::span(&otherwise)=>compile_error!(
                    r#"Bad argument, expected #[ffi_export("...")] for an extern "..." declaration"#
                );
            }),
        },
    });
    let block = fn_decl.block;
    fn_decl.block = Box::new(parse_quote! {
        {
            struct NoUnwind;
            impl ::core::ops::Drop for NoUnwind { fn drop (self: &'_ mut Self) {
                ::std::process::abort();
            }}
            let guard = NoUnwind;
            let ret = (move || #block)();
            ::core::mem::forget(guard);
            ret
        }
    });
    TokenStream::from(::quote::quote!{
        #fn_decl
    })
}

