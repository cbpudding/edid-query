fn main() {
    #[cfg(unix)]
    println!("cargo:rustc-link-lib=X11");
    #[cfg(unix)]
    println!("cargo:rustc-link-lib=Xrandr");
}