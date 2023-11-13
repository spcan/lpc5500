//! Framework macros.



#[macro_export]
macro_rules! entry {
    ($function:ident) => {
        #[no_mangle]
        pub static USERMAIN: fn() -> ! = $function;
    };
}


