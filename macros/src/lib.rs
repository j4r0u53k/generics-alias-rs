use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, Token};
use syn::{Generics, Ident, Item, ItemTrait};

struct GenericsDef {
    ident: Ident,
    generics: Generics,
}

impl Parse for GenericsDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let mut generics: Generics = input.parse()?;
        if input.peek(Token![where]) {
            *generics.make_where_clause() = input.parse()?;
        }
        Ok(Self { ident, generics })
    }
}

/// Defines a new alias for generics.
///
/// The generics is a comma separated list enclosed in `< >` optionally followed by `where`
/// keyword followed by further bounds definitions, each separated by a comma.
///
/// # Examples
///
/// ```
/// # use generics_alias_macros::*;
/// # use macro_magic::export_tokens_no_emit;
/// generics_def!(MyBounds <X: Copy + Clone, Y> where Option<Y>: Send + Sync);
/// ```
#[proc_macro]
pub fn generics_def(input: TokenStream) -> TokenStream {
    let GenericsDef { ident, generics } = parse_macro_input!(input);
    let where_clause = &generics.where_clause;
    quote!(
        #[export_tokens_no_emit]
        trait #ident #generics #where_clause {}
    )
    .into()
}

/// Applies generics defined by [`generics_def`](macro@generics_def) macro to the attached item.
///
/// Accepts a comma separated list of identifiers.
///
/// # Examples
///
/// ```
/// # use generics_alias_macros::*;
/// # use macro_magic::export_tokens_no_emit;
/// # use core::fmt::Debug;
/// # generics_def!(Bounds1 <A: Copy>);
/// # generics_def!(Bounds2 <B: Clone>);
/// # generics_def!(Bounds3 <C: ?Sized + Debug>);
/// #[generics(Bounds1, Bounds2, Bounds3)]
/// trait Foo {
///    // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn generics(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let input_args =
        parse_macro_input!(input with Punctuated<Ident, Comma>::parse_separated_nonempty)
            .into_iter();
    let annotated_item: Item = parse_macro_input!(annotated_item);
    quote!(
    #(#[generics_inner(#input_args)])
    *
    #annotated_item
    )
    .into()
}

#[doc(hidden)]
#[::macro_magic::import_tokens_attr]
#[proc_macro_attribute]
pub fn generics_inner(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(annotated_item as Item);
    let imported = parse_macro_input!(input as ItemTrait);

    let item_generics = match &mut item {
        Item::Fn(fun) => &mut fun.sig.generics,
        Item::Impl(impl_item) => &mut impl_item.generics,
        Item::Struct(struct_item) => &mut struct_item.generics,
        Item::Trait(trait_item) => &mut trait_item.generics,
        _ => panic!("Unsupported type. Types currently supported are: Fn, Impl, Struct, Trait"),
    };

    imported
        .generics
        .params
        .iter()
        .for_each(|b| item_generics.params.push(b.clone()));

    if let Some(imported_where) = &imported.generics.where_clause {
        let item_where = item_generics.make_where_clause();
        imported_where
            .predicates
            .iter()
            .for_each(|p| item_where.predicates.push(p.clone()));
    }
    item.into_token_stream().into()
}
