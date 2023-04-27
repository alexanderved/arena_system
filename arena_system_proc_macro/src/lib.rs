extern crate proc_macro;

mod getter;
mod handleable;
mod handle;
mod util;

use handleable::*;
use handle::*;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Handleable, attributes(handle_getter))]
pub fn derive_handleable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let handleable_info = HandleableInfo::parse(input);
    let handle_info = HandleInfo::parse(&handleable_info);

    let handleable_impl = handleable_info.quote_impl();
    let handle = match handle_info.quote() {
        Ok(h) => h,
        Err(err) => return err.to_compile_error().into(),
    };

    quote! {
        #handleable_impl

        #handle
    }
    .into()
}
