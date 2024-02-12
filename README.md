# Generics aliasing for Rust

This crate provides proc macros to define aliases for generic parameters and apply them
on functions or other objects.

There are two macros provided:

 - `generics_def!(Ident <Generic1, Generic2, ...>)`: A proc macro that takes an identifier followed by generics definition (enclosed in `<>`) as the argument
 - `#[generics(Ident)]`: An attribute macro with one parameter, which is the identifier defined by `generics_def`. The macro injects the defined generics
 into the definition of the function decorated by this macro

## Example

``` rust
use generics_alias::*;

generics_def!(
    ConnectBounds<
        F: Future<Output = shv::Result<(R, W)>>,
        R: futures::AsyncRead + Send + Unpin,
        W: futures::AsyncWrite + Send + Unpin,
    >
);

#[generics(ConnectBounds)]
async fn connection_task<C>(config: ClientConfig, conn_event_sender: Sender<ConnectionEvent>, connect: C) -> shv::Result<()>
where
    C: FnOnce(String) -> F + Clone
{
    // ...
    connection_loop(config, conn_event_sender, connect.clone()).await?;
    // ...
}


#[generics(ConnectBounds)]
async fn connection_loop<C>(config: &ClientConfig, conn_event_sender: &Sender<ConnectionEvent>, connect: C) -> shv::Result<()>
where
    C: FnOnce(String) -> F,
{
    let url = Url::parse(&config.url)?;
    connect(url).await?;
    // ...
}
```


## Details

The crate uses features of [macro_magic](https://github.com/sam0x17/macro_magic.git) crate to export and import tokens between the macros.

`generics_def` basically defines a trait with attached generics and exports its tokens that are later applied whenever `generics` macro is invoked.
Very simple.

The reason for such construct is that generics cannot exist on its own as a valid syntactic construct, so using a trait as its holder is probably
the most convenient choice, easier than e.g. importing and parsing a module holding a variable with a string, or so.

## Usage notes

Users should bring items exported by this crate into scope:

```
use generics_alias::*;
```

Even though this crate reexports `macro_magic` as a dependency, users still have to include it in their own `Cargo.toml` or else the build will
fail with an error:

```
could not find `macro_magic` in the list of imported crates`
```

I couldn't figure out how to cope with this issue :(
