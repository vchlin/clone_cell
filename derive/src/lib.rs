use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use synstructure::{decl_derive, AddBounds, Structure};

decl_derive!([PureClone] => derive_pure_clone);

fn derive_pure_clone(mut s: Structure) -> TokenStream {
    s.underscore_const(true);
    let body = s.each_variant(|v| {
        let bindings = v.bindings();
        v.construct(|_, i| {
            let b = &bindings[i];
            quote! { core::clone::Clone::clone(#b) }
        })
    });
    s.add_bounds(AddBounds::Fields);
    let clone_impl = s.gen_impl(quote! {
        gen impl core::clone::Clone for @Self {
            fn clone(&self) -> Self {
                match *self {
                    #body
                }
            }
        }
    });
    s.add_bounds(AddBounds::None);
    let tys: Vec<_> = s
        .variants()
        .iter()
        .flat_map(|v| v.ast().fields.iter().map(|f| f.ty.clone()))
        .collect();
    for ty in tys {
        // TODO: Proper span
        s.add_where_predicate(parse_quote! { #ty: clone_cell::clone::PureClone });
    }
    let pure_clone_impl = s.gen_impl(quote! {
        gen unsafe impl clone_cell::clone::PureClone for @Self {}
    });
    let output = quote! {
        #clone_impl

        #pure_clone_impl
    };
    //println!("{}", output.to_string());
    output
}
