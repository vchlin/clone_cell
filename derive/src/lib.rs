use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use synstructure::{decl_derive, AddBounds, Structure};

decl_derive!([PureClone] => derive_pure_clone);

fn derive_pure_clone(mut s: Structure) -> TokenStream {
    s.underscore_const(true);
    s.add_bounds(AddBounds::Fields);
    let body = s.each_variant(|v| {
        let bindings = v.bindings();
        v.construct(|_, i| {
            let b = &bindings[i];
            // TODO: Proper span
            quote! { core::clone::Clone::clone(#b) }
        })
    });
    // XXX: Asserts are used instead of adding additional `where` clauses on the `PureClone` impl
    // below. This is because `where` clauses that contain the `Self` type can easily lead to
    // overflowing evaluating trait requirements.
    let asserts = s.variants().iter().flat_map(|v| {
        v.ast().fields.iter().map(|f| {
            let ty = &f.ty;
            let span = ty.span();
            quote_spanned! {span=>
                let _ = <#ty as clone_cell::clone::PureClone>::pure_clone;
            }
        })
    });
    let output = s.gen_impl(quote! {
        gen impl core::clone::Clone for @Self {
            fn clone(&self) -> Self {
                match *self {
                    #body
                }
            }
        }

        gen unsafe impl clone_cell::clone::PureClone for @Self {
            #[inline]
            fn pure_clone(&self) -> Self {
                #(#asserts)*

                core::clone::Clone::clone(self)
            }
        }
    });
    output
}
