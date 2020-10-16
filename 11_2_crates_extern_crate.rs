// Link to `library`, import items under the `rary` module
extern crate rary;

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}

// # Where library.rlib is the path to the compiled library, assumed that it's
// # in the same directory here:
// $ rustc 11_2_crates_extern_crate.rs --extern rary=library.rlib && ./executable
// called rary's `public_function()`
// called rary's `indirect_access()`, that
// > called rary's `private_function()`