fn main() {
    println!(
        "cargo::rustc-env=DATABASE_URL={}",
        std::env::var("DATAANS_WEB_SERVER_DATABASE_URL").unwrap(),
    );
    println!("cargo::rerun-if-changed=build.rs");
}
