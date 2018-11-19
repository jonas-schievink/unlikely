//! Provides an `unlikely!` macro to mark rarely executed code for the
//! optimizer.
//!
//! This also works on stable Rust and for `#[no_std]` applications.

#![doc(html_root_url = "https://docs.rs/unlikely/0.1.0")]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

/// Hints that the given expression will only rarely be evaluated.
///
/// This provides a hint to LLVM that the code inside the macro will not be
/// called often. It can allow LLVM to rearrange code in a way that will improve
/// performance.
///
/// It can be used in a similar fashion to `likely`/`unlikely` branch hints you
/// might be familiar with from C.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # #[macro_use]
/// # extern crate unlikely;
/// # struct Vec<T> { len: usize, capacity: usize, data: [T; 1] }
/// # impl<T> Vec<T> {
/// # fn reallocate(&mut self) -> Result<(), ()> { Ok(()) }
/// pub fn push(&mut self, item: T) {
///     if self.len == self.capacity {
///         // out of space, get more:
///         unlikely! {
///             let result = self.reallocate();
///             result.unwrap();
///         }
///     }
///
///     // TODO: store `item`
/// }
/// # }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! unlikely {
    ( $($t:tt)* ) => {{
        fn _id<T>(t: T) -> T { t }
        _id(#[cold] || {
            $($t)*
        })()
    }};
}

/*

How It Works:

The gist of this is that we assume that LLVM is reasonably good at turning the
`cold` attribute into branch hint information. This seems like a reasonably safe
bet without much risk for regressions, because the panic machinery also relies
on it to not be *too much* in the way of the "real" program code.

We put the "unlikely" code into a closure (allowing use of outer variables)
that's then marked as `#[cold]`. However, you can't actually put attributes on
closures, because attributes on expressions are unstable. HOWEVER however, this
was once accidentally stabilized when the expression is a function argument
(https://github.com/rust-lang/rust/issues/32796), so we pass the closure through
a generic identity function to be able to slap `#[cold]` on it.

*/
