use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{spanned::Spanned, Data, DeriveInput, Error};

pub fn expand_derive_pure_clone(item: &DeriveInput) -> Result<TokenStream, Vec<Error>> {
    let name = &item.ident;

    let fields = match &item.data {
        Data::Struct(s) => s.fields.iter().map(|f| (f.ident.as_ref().unwrap(), &f.ty)),
        _ => todo!(),
    };

    let fields = fields.map(|(ident, ty)| {
        let ident = format_ident!("_Assert{}{}", name, ident);
        let span = ty.span();
        quote_spanned! {span=>
            struct #ident where #ty: PureClone;
        }
    });

    let lib = crate::lib_path();
    let output = quote! {
        #(#fields)*

        unsafe impl #lib::clone::PureClone for #name {}
    };
    Ok(output)
}
