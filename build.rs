use std::path::Path;

fn main() {
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new("src").display()
    );
}
