use proc_macro::TokenStream;
use macro_magic::import_tokens_attr;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input;
use syn::{ItemTrait, Item, ItemFn, Ident, Generics};
use syn::punctuated::Punctuated;
use syn::token::Comma;
// use syn::Token;

struct GenericsDef {
    ident: Ident,
    generics: Generics,
}

impl Parse for GenericsDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        // input.parse::<Token![,]>()?;
        let generics: Generics = input.parse()?;
        Ok(Self { ident, generics })
    }
}

#[proc_macro]
pub fn generics_def(input: TokenStream) -> TokenStream {
    let GenericsDef { ident, generics } = parse_macro_input!(input);
    quote!(
    #[export_tokens_no_emit]
    trait #ident #generics {}
    )
    .into()
}

#[proc_macro_attribute]
pub fn generics(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let input_args  = parse_macro_input!(input with Punctuated<Ident, Comma>::parse_separated_nonempty).into_iter();
    let annotated_item: Item = parse_macro_input!(annotated_item);
    quote!(
        #(#[generics_inner(#input_args)])
        *
        #annotated_item
        ).into()
}

#[import_tokens_attr]
#[proc_macro_attribute]
pub fn generics_inner(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let mut fun = parse_macro_input!(annotated_item as ItemFn);
    let imported = parse_macro_input!(input as ItemTrait);
    let bounds = imported.generics;
    let generic_params = &mut fun.sig.generics.params;
    bounds
        .params
        .iter()
        .for_each(|b| generic_params.push(b.clone()));
    fun.into_token_stream().into()
}
