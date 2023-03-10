// build.rs
fn main() {
    let v = option_env!("CARGO_APP_VERSION").unwrap_or("dev");
    println!("cargo:rustc-env=CARGO_APP_VERSION={v}");
}
