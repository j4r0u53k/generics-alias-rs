//! # Generics alias
//!
//! The crate provides macros for creating an alias of some generics that can be reused
//! on functions or struct/trait impls without repeating the whole definition.
//!
//! ## Examples
//!
//! Making an alias for multiple trait bounds:
//!
//! ```
//! use generics_alias::*;
//!
//! generics_def!(ManyBounds
//!     <
//!         F: FnOnce(Y) -> Z,
//!         Y: Copy,
//!         Z: Into<Option<Y>>,
//!     >
//! );
//!
//! #[generics(ManyBounds)]
//! fn foo(f: F, y: Y) -> Z {
//!    f(y)
//! }
//! ```
//!
//! Use on a struct/trait and impl block:
//!
//! ```
//! use core::fmt::Debug;
//! use generics_alias::*;
//!
//! generics_def!(DebugClonable <X: Debug + Clone>);
//!
//! #[generics(DebugClonable)]
//! struct Foo { x: X }
//!
//! #[generics(DebugClonable)]
//! impl Foo<X> { }
//!
//! #[generics(DebugClonable)]
//! trait Bar { }
//!
//! #[generics(DebugClonable)]
//! impl Bar<X> for i32 { }
//! ```
//!
//! Where clause:
//!
//! ```
//! use core::fmt::Debug;
//! use core::fmt::Display;
//! use generics_alias::*;
//!
//! generics_def!(WhereBounds <X: Display, T: Debug> where Option<T>: Debug);
//!
//! #[generics(WhereBounds)]
//! fn foo(x: X, t: Option<T>) {
//!    println!("x={}, t={:?}", x, t);
//! }
//! foo(42, Some(1));
//! ```
//!

// NOTE: The reexport of `macro_magic` crate does not work (I do not know why), so users still
// have to specify the dependency in their Cargo.toml :(
pub extern crate macro_magic;
#[doc(hidden)]
pub use macro_magic::*;
pub use generics_alias_macros::*;


#[cfg(test)]
mod tests {
    use core::fmt::Debug;
    use std::marker::PhantomData;
    use super::*;

    generics_def!(DebugClonable <X: Debug + Clone>);

    #[test]
    fn fn_simple() {
        #[generics(DebugClonable)]
        fn foo(x: X) {
            println!("x: {:?}", x.clone());
        }
        let x = 42;
        foo(x);
    }

    #[test]
    fn impl_simple() {
        #[allow(dead_code)]
        #[generics(DebugClonable)]
        struct Foo {
            _phantom_data: PhantomData<X>,
        }

        #[generics(DebugClonable)]
        impl Foo<X> {
        }
    }

    #[test]
    fn trait_simple() {
        #[allow(dead_code)]
        #[generics(DebugClonable)]
        trait Foo { }

        #[generics(DebugClonable)]
        impl Foo<X> for i32 {
        }
    }

    #[test]
    fn impl_fn_simple() {
        struct Foo;
        impl Foo {
            #[generics(DebugClonable)]
            fn foo(x: X) {
                println!("x: {:?}", x.clone());
            }
        }
        let x = 42;
        Foo::foo(x);
    }
}
