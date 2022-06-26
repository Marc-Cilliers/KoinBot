#[macro_use]
extern crate lazy_static;

mod utils;
use std::env;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1"); //Uncomment for debugging
    utils::discord::main()
}
