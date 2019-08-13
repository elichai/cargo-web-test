extern crate cc;

use std::env;

fn main() {
    let mut base_config = cc::Build::new();
    base_config
        .include("depend/")
        .file("depend/linkme.c")
        .compile("linkme");
}
