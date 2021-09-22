use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Ident, Path, Type};

#[proc_macro_derive(PureClone)]
pub fn derive_pure_clone(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    expand_derive_pure_clone(&item).into()
}

fn lib_path() -> Path {
    format_ident!("clone_cell").into()
}

fn expand_derive_pure_clone(item: &DeriveInput) -> TokenStream {
    let name = &item.ident;

    let fields = match &item.data {
        Data::Struct(s) => s.fields.iter().map(|f| (f.ident.as_ref().unwrap(), &f.ty)),
        _ => todo!(),
    };

    let asserts = fields.clone().map(|(ident, ty)| {
        let ident = format_ident!("_Assert{}{}", name, ident);
        let span = ty.span();
        quote_spanned! {span=>
            struct #ident where #ty: PureClone;
        }
    });

    let pure_clone_impl = impl_pure_clone(name, fields);

    quote! {
        const _: () = {
            #(#asserts)*

            #pure_clone_impl
        };
    }
}

fn impl_pure_clone<'a>(
    name: &Ident,
    fields: impl Iterator<Item = (&'a Ident, &'a Type)>,
) -> TokenStream {
    // `Clone` must be implemented by us for it to be trusted.
    let clone_impl = impl_clone(name, fields);
    let lib = lib_path();
    quote! {
        #clone_impl

        unsafe impl #lib::clone::PureClone for #name {}
    }
}

fn impl_clone<'a>(
    name: &Ident,
    fields: impl Iterator<Item = (&'a Ident, &'a Type)>,
) -> TokenStream {
    let fields = fields.map(|(ident, _)| {
        quote! {
            #ident: core::clone::Clone::clone(&self.#ident)
        }
    });
    quote! {
        impl core::clone::Clone for #name {
            fn clone(&self) -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    }
}
