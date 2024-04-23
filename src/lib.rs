use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn debug_only(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let name = input.sig.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.sig.generics.split_for_impl();
    let inputs = input.sig.inputs.iter();
    let output = match input.sig.output {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, ref ty) => quote!(#ty),
    };
    let expanded = if cfg!(debug_assertions) {
        quote! {
            #[allow(dead_code)]
            #input
        }
    } else {
        quote! {
            #[allow(dead_code)]
            fn #name #impl_generics #ty_generics (#(#inputs),*) -> #output {
            }
        }
    };
    TokenStream::from(expanded)
}
