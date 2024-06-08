extern crate proc_macro;
use proc_macro::TokenStream;

#[no_mangle]
pub extern "C" fn and() -> TokenStream {
    "fn answer() -> u8 { 32 }".parse().unwrap()
}

