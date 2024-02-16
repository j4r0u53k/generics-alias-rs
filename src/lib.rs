// NOTE: The reexport of `macro_magic` crate does not work (I do not know why), so users still
// have to specify the dependency in their Cargo.toml :(
pub extern crate macro_magic;
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
