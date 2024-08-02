fn main() {
    println!("cargo::rustc-check-cfg=cfg(windows_is_host_os)");
    if cfg!(windows) {
        println!("cargo:rustc-cfg=windows_is_host_os");
    }
}