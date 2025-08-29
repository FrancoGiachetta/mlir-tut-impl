fn main() {
    // Point to the actual location where CMake built the library
    println!("cargo:rustc-link-search=native=../build-dir/lib/Dialect");
    println!("cargo:rustc-link-lib=dialect_bindings");
}
