use copy_dir::copy_dir;

use std::env;
use std::fs::{copy, remove_dir_all};
use std::path::Path;
use std::process::Command;

fn main() {
    let build = Build::new();

    if let Ok(ipasir) = env::var("IPASIR") {
        build.copy_existing_library(ipasir);
    }

    if !build.static_library_exists() {
        build.remove_cadical_dir();
        build.copy_cadical_dir();

        build.configure_cadical();
        build.make_cadical();

        build.copy_cadical_library();
        build.remove_cadical_dir();
    }

    println!("cargo:rustc-link-search={}", build.out_dir);
    println!("cargo:rustc-link-lib=static=ipasir");

    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=stdc++");
}

struct Build {
    pub out_dir: String,
}

impl Build {
    fn new() -> Self {
        Self { out_dir: env::var("OUT_DIR").unwrap() }
    }

    fn static_library_exists(&self) -> bool {
        Path::new(&format!("{}/libipasir.a", self.out_dir)).exists()
    }

    fn remove_cadical_dir(&self) {
        let _ = remove_dir_all(format!("{}/cadical", self.out_dir));
    }

    fn copy_cadical_dir(&self) {
        copy_dir("vendor/cadical", format!("{}/cadical", self.out_dir)).unwrap();
    }

    fn configure_cadical(&self) {
        Command::new("./configure")
            .current_dir(format!("{}/cadical", self.out_dir))
            .spawn().unwrap().wait().unwrap();
    }

    fn make_cadical(&self) {
        Command::new("make")
            .current_dir(format!("{}/cadical/build", self.out_dir))
            .spawn().unwrap().wait().unwrap();
    }

    fn copy_cadical_library(&self) {
        copy(
            format!("{}/cadical/build/libcadical.a", self.out_dir),
            format!("{}/libipasir.a", self.out_dir)
        ).unwrap();
    }

    fn copy_existing_library(&self, ipasir: String) {
        copy(ipasir, format!("{}/libipasir.a", self.out_dir)).unwrap();
    }
}
