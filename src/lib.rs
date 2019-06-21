pub use ::ffi_export_proc_macro::ffi_export;

#[doc(hidden)]
pub use ::ffi_export_proc_macro::named_hack;

mod ffi_export {
    pub use super::{
        AbortOnDrop,
        named_hack,
    };
}

#[doc(hidden)] pub
struct AbortOnDrop;

impl Drop for AbortOnDrop { fn drop (self: &'_ mut Self) {
    ::std::process::abort();
}}

