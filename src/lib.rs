#![feature(const_fn)]
#[macro_export]
macro_rules! const_ft {
    (pub fn $fn_name:ident($($arg:ident : $typ:ty),*) -> $ret: ty $body: block ) => {
        #[cfg(feature = "const_fn")]
        pub const fn $fn_name($($arg : $typ,)*) -> $ret $body

        #[cfg(not(feature = "const_fn"))]
        pub fn $fn_name($($arg : $typ,)*) -> $ret $body
    }
}


#[cfg(test)]
mod tests {
    const_ft! {
            pub fn foo() -> u32 {
                1u32
            }
        }
    #[test]
    fn it_works() {
        assert_eq!(foo(), 1u32);
        assert_eq!(foo(1u32), 1u32);

    }
}
