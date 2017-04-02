#![forbid(exceeding_bitshifts, mutable_transmutes, no_mangle_const_items,
          unknown_crate_types, warnings)]
#![deny(bad_style, deprecated, improper_ctypes,
        non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, stable_features, unconditional_recursion,
        unknown_lints, unsafe_code, unused, unused_allocation, unused_attributes,
        unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]
#![allow(box_pointers, fat_ptr_transmutes, missing_copy_implementations,
         missing_debug_implementations, variant_size_differences, non_camel_case_types)]

#[macro_use]
extern crate unwrap;

#[cfg(not(feature = "get-libsodium"))]
extern crate pkg_config;

const VERSION: &'static str = "1.0.11";

#[cfg(not(feature = "get-libsodium"))]
fn main() {
    use std::env;
    if let Ok(lib_dir) = env::var("SODIUM_LIB_DIR") {
        println!("cargo:rustc-flags=-L native={}", lib_dir);
        let mode = match env::var_os("SODIUM_STATIC") {
            Some(_) => "static",
            None => "dylib",
        };
        println!("cargo:rustc-flags=-l {0}=sodium", mode);
        println!("cargo:warning=Using unknown libsodium version.  This crate is tested against \
                  {} and may not be fully compatible with other versions.",
                 VERSION);
    } else {
        let lib_details = unwrap!(pkg_config::probe_library("libsodium"));
        if lib_details.version != VERSION {
            println!("cargo:warning=Using libsodium version {}.  This crate is tested against {} \
                      and may not be fully compatible with {}.",
                     lib_details.version,
                     VERSION,
                     lib_details.version);
        }
    }
}



#[cfg(feature = "get-libsodium")]
extern crate gcc;
#[cfg(feature = "get-libsodium")]
extern crate flate2;
#[cfg(feature = "get-libsodium")]
extern crate reqwest;
#[cfg(feature = "get-libsodium")]
extern crate tar;

#[cfg(feature = "get-libsodium")]
pub fn download_libsodium() -> tar::Archive<flate2::read::GzDecoder<reqwest::Response>> {
    use flate2::read::GzDecoder;
    use reqwest;
    use tar::Archive;

    let basename = "libsodium-".to_string() + VERSION;
    let gz_filename = if cfg!(windows) {
        basename.clone() + "-mingw.tar.gz"
    } else {
        basename.clone() + ".tar.gz"
    };
    let url = "https://download.libsodium.org/libsodium/releases/".to_string() + &gz_filename;
    let response = unwrap!(reqwest::get(&url));
    if !response.status().is_success() {
        panic!("\nFailed to download libsodium.\nURL: {}\nResponse status: {}\nResponse \
                   headers:\n{}",
               url,
               response.status(),
               response.headers());
    }
    let gz_decoder = unwrap!(GzDecoder::new(response));
    Archive::new(gz_decoder)
}

#[cfg(all(windows, feature = "get-libsodium"))]
fn main() {
    use std::{env, fs};
    use std::path::{Path, PathBuf};
    use std::process::Command;

    if cfg!(target_env = "msvc") {
        panic!("This feature currently can't be used with MSVC builds.");
    }

    // Download and unpack libsodium.  Extract just the appropriate version of libsodium.a and
    // headers to the install path
    let mut archive = download_libsodium();
    let install_dir = unwrap!(env::var("OUT_DIR")) + "/installed";
    unwrap!(fs::create_dir_all(Path::new(&install_dir).join("lib")));
    let arch_path = if cfg!(target_pointer_width = "32") {
        Path::new("libsodium-win32")
    } else if cfg!(target_pointer_width = "64") {
        Path::new("libsodium-win64")
    } else {
        panic!("target_pointer_width not 32 or 64")
    };

    let unpacked_include = arch_path.join("include");
    let unpacked_lib = arch_path.join("lib\\libsodium.a");
    let entries = unwrap!(archive.entries());
    for entry_result in entries {
        let mut entry = unwrap!(entry_result);
        let entry_path = unwrap!(entry.path()).to_path_buf();
        let full_install_path = if entry_path.starts_with(&unpacked_include) {
            let include_file = unwrap!(entry_path.strip_prefix(arch_path));
            Path::new(&install_dir).join(include_file)
        } else if entry_path == unpacked_lib {
            Path::new(&install_dir).join("lib").join("libsodium.a")
        } else {
            continue;
        };
        let _ = unwrap!(entry.unpack(full_install_path));
    }

    // Get path to gcc in order to guess location of libpthread.a
    let mut lib_search_dirs = vec![Path::new(&install_dir).join("lib")];
    let mut where_cmd = Command::new("where");
    let where_output = where_cmd
        .arg(gcc::Config::new().get_compiler().path())
        .output()
        .unwrap_or_else(|error| {
                            panic!("Failed to run where command: {}", error);
                        });
    if !where_output.status.success() {
        panic!("\n{:?}\n{}\n{}\n",
               where_cmd,
               String::from_utf8_lossy(&where_output.stdout),
               String::from_utf8_lossy(&where_output.stderr));
    }
    let compiler_path_as_string = String::from_utf8_lossy(&where_output.stdout);
    let compiler_path = PathBuf::from(unwrap!(compiler_path_as_string.lines().next()).trim());
    let mingw_path = unwrap!(unwrap!(compiler_path.parent()).parent());
    if cfg!(target_pointer_width = "32") {
        lib_search_dirs.push(mingw_path.join("lib"));
        lib_search_dirs.push(mingw_path.join("i686-w64-mingw32").join("lib"));
    } else {
        lib_search_dirs.push(mingw_path.join("x86_64-w64-mingw32").join("lib"));
    }

    println!("cargo:rustc-link-lib=static=sodium");
    println!("cargo:rustc-link-lib=pthread");
    for lib_search_dir in &lib_search_dirs {
        println!("cargo:rustc-link-search=native={}",
                 lib_search_dir.display());
    }
    println!("cargo:include={}/include", install_dir);
}



#[cfg(all(not(windows), feature = "get-libsodium"))]
fn main() {
    use std::env;
    use std::process::Command;

    // Download and unpack libsodium
    let mut archive = download_libsodium();

    // Avoid issues with paths containing spaces by falling back to using /tmp
    let mut install_dir = unwrap!(env::var("OUT_DIR")) + "/installed";
    let mut source_dir = unwrap!(env::var("OUT_DIR")) + "/source";
    let target = unwrap!(env::var("TARGET"));
    if install_dir.contains(" ") {
        let fallback_path = "/tmp/libsodium-".to_string() + &VERSION + "/" + &target;
        install_dir = fallback_path.clone() + "/installed";
        source_dir = fallback_path.clone() + "/source";
        println!("cargo:warning=The path to the usual build directory contains spaces and hence \
                  can't be used to build libsodium.  Falling back to use {}.  If running `cargo \
                  clean`, ensure you also delete this fallback directory",
                 fallback_path);
    }
    unwrap!(archive.unpack(&source_dir));
    source_dir.push_str(&format!("/libsodium-{}", VERSION));

    // Run `./configure`
    let gcc = gcc::Config::new();
    let (cc, cflags) = if target.contains("i686") {
        (format!("{} -m32", gcc.get_compiler().path().display()),
         env::var("CFLAGS").unwrap_or(String::default()) + " -march=i686")
    } else {
        (format!("{}", gcc.get_compiler().path().display()),
         env::var("CFLAGS").unwrap_or(String::default()))
    };
    let prefix_arg = format!("--prefix={}", install_dir);
    let host = unwrap!(env::var("HOST"));
    let host_arg = format!("--host={}", target);
    let cross_compiling = target != host;
    let help = if cross_compiling {
        "***********************************************************\n\
         Possible missing dependencies.\n\
         See https://github.com/maidsafe/rust_sodium#cross-compiling\n\
         ***********************************************************\n\n"
    } else {
        ""
    };

    let mut configure_cmd = Command::new("./configure");
    let configure_output = configure_cmd
        .current_dir(&source_dir)
        .env("CC", &cc)
        .env("CFLAGS", &cflags)
        .arg(&prefix_arg)
        .arg(&host_arg)
        .arg("--enable-shared=no")
        .arg("--disable-pie")
        .output()
        .unwrap_or_else(|error| {
                            panic!("Failed to run './configure': {}\n{}", error, help);
                        });
    if !configure_output.status.success() {
        panic!("\n{:?}\nCFLAGS={}\nCC={}\n{}\n{}\n{}\n",
               configure_cmd,
               cflags,
               cc,
               String::from_utf8_lossy(&configure_output.stdout),
               String::from_utf8_lossy(&configure_output.stderr),
               help);
    }

    // Run `make check`, or `make all` if we're cross-compiling
    let j_arg = format!("-j{}", unwrap!(env::var("NUM_JOBS")));
    let make_arg = if cross_compiling { "all" } else { "check" };
    let mut make_cmd = Command::new("make");
    let make_output = make_cmd
        .current_dir(&source_dir)
        .env("V", "1")
        .arg(make_arg)
        .arg(&j_arg)
        .output()
        .unwrap_or_else(|error| {
                            panic!("Failed to run 'make {}': {}\n{}", make_arg, error, help);
                        });
    if !make_output.status.success() {
        panic!("\n{:?}\n{}\n{}\n{}\n{}",
               make_cmd,
               String::from_utf8_lossy(&configure_output.stdout),
               String::from_utf8_lossy(&make_output.stdout),
               String::from_utf8_lossy(&make_output.stderr),
               help);
    }

    // Run `make install`
    let mut install_cmd = Command::new("make");
    let install_output = install_cmd
        .current_dir(&source_dir)
        .arg("install")
        .output()
        .unwrap_or_else(|error| {
                            panic!("Failed to run 'make install': {}", error);
                        });
    if !install_output.status.success() {
        panic!("\n{:?}\n{}\n{}\n{}\n{}\n",
               install_cmd,
               String::from_utf8_lossy(&configure_output.stdout),
               String::from_utf8_lossy(&make_output.stdout),
               String::from_utf8_lossy(&install_output.stdout),
               String::from_utf8_lossy(&install_output.stderr));
    }

    println!("cargo:rustc-link-lib=static=sodium");
    println!("cargo:rustc-link-search=native={}/lib", install_dir);
    println!("cargo:include={}/include", install_dir);
}
