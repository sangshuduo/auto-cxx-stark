use autocxx::prelude::*;
include_cpp! {
    #include "input.h"
    safety!(unsafe_ffi)
    generate!("hello_world")
    generate!("libstark::BairWitness")
    generate!("Algebra::FieldElement")
}

fn main() {
    println!("Hello, world!");
    ffi::hello_world();
}
