//! Default traits for a plain data object
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_results)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::let_underscore_untyped)]
#![allow(clippy::similar_names)]

use proc::quote::quote;

/// Default traits for a plain data object
#[proc::attribute]
pub fn value_type(
    #[allow(non_snake_case)] Copy: proc::meta::Switch,
    input: proc::DeriveInput,
) -> proc::Result<proc::TokenStream> {
    let copy = if Copy {
        Some(quote!(#[derive(Copy)]))
    } else {
        None
    };
    Ok(quote! {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #copy
        #input
    })
}
