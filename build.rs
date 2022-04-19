use std::path::Path;

fn main() {
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new("../halide_test_app/src/gens").display()
    );
}
