fn main() {
    println!("cargo:rustc-link-lib=dylib=stdc++");

    println!("cargo:rustc-link-search=libs");
}
