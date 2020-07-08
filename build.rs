use bindgen::Builder;
use copy_dir::copy_dir;

use std::env;
use std::fs::{copy, remove_dir_all, remove_file};
use std::path::Path;
use std::process::Command;

fn main() {
    let build = Build::new();

    build.generate_bindings();

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

    build.static_link_to_ipasir();
    build.dynamic_link_to_cpp_stdlib();
}

struct Build {
    pub out_dir: String,
}

impl Build {
    fn new() -> Self {
        Self { out_dir: env::var("OUT_DIR").unwrap() }
    }

    fn generate_bindings(&self) {
        Builder::default()
            .header("vendor/ipasir/ipasir.h").generate().unwrap()
            .write_to_file(self.path("bindings.rs")).unwrap();
    }

    fn static_library_exists(&self) -> bool {
        Path::new(&self.ipasir_out()).exists()
    }

    fn remove_cadical_dir(&self) {
        let _ = remove_dir_all(self.path("cadical"));
    }

    fn copy_cadical_dir(&self) {
        copy_dir("vendor/cadical", self.path("cadical")).unwrap();
    }

    fn configure_cadical(&self) {
        Command::new(self.path("cadical/configure"))
            .arg("-fPIC")
            .current_dir(self.path("cadical"))
            .spawn().unwrap().wait().unwrap();
    }

    fn make_cadical(&self) {
        Command::new("make")
            .current_dir(self.path("cadical/build"))
            .spawn().unwrap().wait().unwrap();
    }

    fn copy_cadical_library(&self) {
        if self.static_library_exists() {
            remove_file(self.ipasir_out()).unwrap();
        }
        copy(
            self.path("cadical/build/libcadical.a"),
            self.ipasir_out(),
        ).unwrap();
    }

    fn copy_existing_library(&self, ipasir: String) {
        if self.static_library_exists() {
            remove_file(self.ipasir_out()).unwrap();
        }
        copy(ipasir, self.ipasir_out()).unwrap();
    }

    fn static_link_to_ipasir(&self) {
        println!("cargo:rustc-link-search={}", self.out_dir);
        println!("cargo:rustc-link-lib=static=ipasir");
    }

    // The C++ standard library name varies by platform.
    // Based on: https://docs.rs/cc/1.0.57/src/cc/lib.rs.html#2178
    fn dynamic_link_to_cpp_stdlib(&self) {
        let target = env::var("TARGET").unwrap();

        let name = if target.contains("msvc") {
            return;
        } else if target.contains("apple") {
            "c++"
        } else if target.contains("freebsd") {
            "c++"
        } else if target.contains("openbsd") {
            "c++"
        } else {
            "stdc++"
        };

        println!("cargo:rustc-link-lib=dylib={}", name);
    }

    fn ipasir_out(&self) -> String {
        self.path("libipasir.a")
    }

    fn path(&self, p: &'static str) -> String {
        format!("{}/{}", self.out_dir, p)
    }
}
