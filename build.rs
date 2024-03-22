fn main() {
    println!("cargo:warning================================== compile build.rs =================================\n");
    if slint_build::compile("ui/startwindow.slint").is_ok() {
        println!(
            "cargo:rustc-env=SLINT_INCLUDE_START={}/startwindow.rs",
            std::env::var("OUT_DIR").unwrap()
        );
        println!("cargo:warning=compile ui/startwindow.slint success\n");
    }
    if slint_build::compile("ui/encryptionwindow.slint").is_ok() {
        println!(
            "cargo:rustc-env=SLINT_INCLUDE_ENCRYPT={}/encryptionwindow.rs",
            std::env::var("OUT_DIR").unwrap()
        );
        println!("cargo:warning=compile ui/encryptionwindow.slint success\n");
    }
    if slint_build::compile("ui/decryptionwindow.slint").is_ok() {
        println!(
            "cargo:rustc-env=SLINT_INCLUDE_DECRYPT={}/decryptionwindow.rs",
            std::env::var("OUT_DIR").unwrap()
        );
        println!("cargo:warning=compile ui/decryptionwindow.slint success\n");
    }
    if slint_build::compile("ui/keygenwindow.slint").is_ok() {
        println!(
            "cargo:rustc-env=SLINT_INCLUDE_KEYGEN={}/keygenwindow.rs",
            std::env::var("OUT_DIR").unwrap()
        );
        println!("cargo:warning=compile ui/keygenwindow.slint success\n");
    }
}
