#![feature(macro_rules, phase)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate simple_oauth;

macro_rules! test(
    ($name:ident $expr:expr) => (
        #[test]
        fn $name() {
            // setup();
            $expr;
        }
    )
)

mod test_header;
