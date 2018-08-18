#![cfg_attr(feature = "const_fn", feature(const_fn))]

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

    #[test]
    fn it_works() {
        assert_eq!(public_with_no_args(), 1u32);
        assert_eq!(public_with_args(1u32), 1u32);
        assert_eq!(private_with_no_args(), 1u32);
        assert_eq!(private_with_args(1u32, 2u32), 1u32);
        assert_eq!(pub_crate_with_args(1u32), 1u32);


        public_with_no_return();
        private_with_no_return(1u32);
    }
}
