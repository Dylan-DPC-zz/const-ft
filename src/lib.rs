#![cfg_attr(feature = "const_fn", feature(const_fn))]
#![no_std]

//! A macro for easy generation and wrapping of a const function under a const_fn feature gate.
//!
//! The macro will generate two functions with the same name - one const fn for when the feature gate
//! `const_fn` is enabled and the other for the all other cases.
//!
//! `const` fns are unstable, and the const_fn feature gate should be enabled only on nightly.
//!
//! The macro accepts a single function block, as an input. To include multiple functions, you have
//! to use separate macro calls. The macro works for public, private and pub(x) functions.
//!
//! To install this crate as a dependency, add it to your Cargo.toml:
//!
//! ```toml
//! const_ft = { version =  "0.1", features = "const_fn" }
//! ```
//!
//! Example:
//!
//! ```rust
//! #![cfg_attr(feature = "const_fn", feature(const_fn))]
//!
//! #[macro_use]
//! extern crate const_ft;
//!
//! const_ft! {
//!      pub fn some_function() -> u32 {
//!         1u32
//!      }
//! }
//!
//!
//! fn main() {
//!     assert_eq!(some_function(), 1u32);
//! }
//! ```
//!

#[macro_export]
macro_rules! const_ft {
    (pub fn $fn_name:ident($($arg:ident : $typ:ty),*) $body: block ) => {
        #[cfg(feature = "const_fn")]
        pub const fn $fn_name($($arg : $typ,)*) $body

        #[cfg(not(feature = "const_fn"))]
        pub fn $fn_name($($arg : $typ,)*) $body
    };
    (pub fn $fn_name:ident($($arg:ident : $typ:ty),*) -> $ret: ty $body: block ) => {
        #[cfg(feature = "const_fn")]
        pub const fn $fn_name($($arg : $typ,)*) -> $ret $body

        #[cfg(not(feature = "const_fn"))]
        pub fn $fn_name($($arg : $typ,)*) -> $ret $body
    };

    (fn $fn_name:ident($($arg:ident : $typ:ty),*) -> $ret: ty $body: block ) => {
        #[cfg(feature = "const_fn")]
        const fn $fn_name($($arg : $typ,)*) -> $ret $body

        #[cfg(not(feature = "const_fn"))]
        fn $fn_name($($arg : $typ,)*) -> $ret $body
    };

    (fn $fn_name:ident($($arg:ident : $typ:ty),*) $body: block ) => {
        #[cfg(feature = "const_fn")]
        const fn $fn_name($($arg : $typ,)*) $body

        #[cfg(not(feature = "const_fn"))]
        fn $fn_name($($arg : $typ,)*) $body
    };

    (pub($scope:ident) fn $fn_name:ident($($arg:ident : $typ:ty),*) -> $ret:ty $body:block ) => {
        #[cfg(feature = "const_fn")]
        pub($scope) const fn $fn_name($($arg : $typ,)*) -> $ret $body

        #[cfg(not(feature = "const_fn"))]
         pub($scope) fn $fn_name($($arg : $typ,)*) -> $ret $body
    };

    (pub($scope:ident) fn $fn_name:ident($($arg:ident : $typ:ty),*) $body:block ) => {
        #[cfg(feature = "const_fn")]
        pub($scope) const fn $fn_name($($arg : $typ,)*) $body

        #[cfg(not(feature = "const_fn"))]
        pub($scope) fn $fn_name($($arg : $typ,)*) $body
    };

    (fn $fn_name:ident( $self_:ident, $($arg:ident : $typ:ty),*) $body: block ) => {
        #[cfg(feature = "const_fn")]
        pub const fn $fn_name($self_:ident, $($arg : $typ,)*) $body

        #[cfg(not(feature = "const_fn"))]
        pub fn $fn_name($self_:ident, $($arg : $typ,)*) $body
    };
    (fn $fn_name:ident($self_:ident, $($arg:ident : $typ:ty),*) -> $ret: ty $body: block ) => {
        #[cfg(feature = "const_fn")]
        pub const fn $fn_name($self_, $($arg : $typ,)*) -> $ret $body

        #[cfg(not(feature = "const_fn"))]
        pub fn $fn_name($self_, $($arg : $typ,)*) -> $ret $body
    };

   (pub fn $fn_name:ident(&$self_: ident, $($arg:ident : $typ:ty),*) -> $ret: ty $body: block ) => {
        #[cfg(feature = "const_fn")]
        pub const fn $fn_name(&$self_, $($arg : $typ,)*) -> $ret $body

        #[cfg(not(feature = "const_fn"))]
        pub fn $fn_name(&$self_, $($arg : $typ,)*) -> $ret $body
    };

    (pub fn $fn_name:ident(&$self_: ident, $($arg:ident : $typ:ty),*) -> $body: block ) => {
        #[cfg(feature = "const_fn")]
        pub const fn $fn_name(&$self_, $($arg : $typ,)*) $body

        #[cfg(not(feature = "const_fn"))]
        pub fn $fn_name(&$self_, $($arg : $typ,)*) $body
    };

}

#[cfg(test)]
mod tests {
    const_ft! {
        pub fn public_with_no_args() -> u32 {
            1u32
        }
    }

    const_ft! {
        pub fn public_with_args(x: u32) -> u32 {
            x
        }
    }

    const_ft! {
            pub fn public_with_no_return() {}
    }

    const_ft! {
    fn private_with_no_args() -> u32 {
        1u32
    }
    }

    const_ft! {
        fn private_with_args(x: u32, _y: u32) -> u32 {
            x
        }
    }

    const_ft! {
        fn private_with_no_return(_x: u32) {}
    }

    const_ft! {
        pub(crate) fn pub_crate_with_args(x: u32) -> u32 {
            x
        }
    }

    struct Foo(u32);
    impl Foo {
        const_ft! {
        pub fn pub_with_self(&self, _x: u32) -> u32 {
            self.0
        }
    }
    }

    #[test]
    pub fn it_works() {
        assert_eq!(public_with_no_args(), 1u32);
        assert_eq!(public_with_args(1u32), 1u32);
        assert_eq!(private_with_no_args(), 1u32);
        assert_eq!(private_with_args(1u32, 2u32), 1u32);
        assert_eq!(pub_crate_with_args(1u32), 1u32);

        public_with_no_return();
        private_with_no_return(1u32);

        let foo = Foo(1u32);
        assert_eq!(foo.pub_with_self(1u32), 1u32);
    }
}

