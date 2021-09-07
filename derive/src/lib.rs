use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Error, Path};

mod clone;

#[proc_macro_derive(PureClone)]
pub fn derive_pure_clone(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    clone::expand_derive_pure_clone(&item)
        .unwrap_or_else(to_compile_errors)
        .into()
}

fn to_compile_errors(errs: Vec<Error>) -> proc_macro2::TokenStream {
    let errs = errs.iter().map(Error::to_compile_error);
    quote!(#(#errs)*)
}

fn lib_path() -> Path {
    format_ident!("clone_cell").into()
}
