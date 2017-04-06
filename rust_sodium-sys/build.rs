#[macro_use]
extern crate unwrap;

#[cfg(feature = "use-installed-libsodium")]
extern crate pkg_config;

const VERSION: &'static str = "1.0.12";

#[cfg(feature = "use-installed-libsodium")]
fn main() {
    use std::env;
    if let Ok(lib_dir) = env::var("SODIUM_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);
        let mode = match env::var_os("SODIUM_STATIC") {
            Some(_) => "static",
            None => "dylib",
        };
        println!("cargo:rustc-link-lib={0}=sodium", mode);
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



#[cfg(not(feature = "use-installed-libsodium"))]
extern crate gcc;
#[cfg(not(feature = "use-installed-libsodium"))]
extern crate flate2;
#[cfg(not(feature = "use-installed-libsodium"))]
extern crate tar;

#[cfg(not(feature = "use-installed-libsodium"))]
fn get_install_dir() -> String {
    use std::env;
    unwrap!(env::var("OUT_DIR")) + "/installed"
}

#[cfg(all(windows, not(feature = "use-installed-libsodium")))]
fn main() {
    use std::fs::{self, File};
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use flate2::read::GzDecoder;
    use tar::Archive;

    if cfg!(target_env = "msvc") {
        panic!("This feature currently can't be used with MSVC builds.");
    }

    // Check the version of Powershell available
    let mut check_ps_version_cmd = Command::new("powershell");
    let check_ps_version_output = check_ps_version_cmd
        .arg("-Command")
        .arg("If ($PSVersionTable.PSVersion.Major -lt 4) { exit 1 }")
        .output()
        .unwrap_or_else(|error| {
                            panic!("Failed to run powershell command: {}", error);
                        });
    if !check_ps_version_output.status.success() {
        panic!("\n{:?}\n{}\n{}\nYou must have Powershell v4.0 or greater installed.\n\n",
               check_ps_version_cmd,
               String::from_utf8_lossy(&check_ps_version_output.stdout),
               String::from_utf8_lossy(&check_ps_version_output.stderr));
    }

    // Download gz tarball
    let basename = "libsodium-".to_string() + VERSION;
    let gz_filename = basename.clone() + "-mingw.tar.gz";
    let url = "https://download.libsodium.org/libsodium/releases/".to_string() + &gz_filename;
    let install_dir = get_install_dir();
    let gz_path = install_dir.clone() + "/" + &gz_filename;
    unwrap!(fs::create_dir_all(Path::new(&install_dir).join("lib")));

    let command = "([Net.ServicePointManager]::SecurityProtocol = 'Tls12') -and \
                   ((New-Object System.Net.WebClient).DownloadFile(\""
            .to_string() + &url + "\", \"" + &gz_path + "\"))";
    let mut download_cmd = Command::new("powershell");
    let download_output = download_cmd
        .arg("-Command")
        .arg(&command)
        .output()
        .unwrap_or_else(|error| {
                            panic!("Failed to run powershell download command: {}", error);
                        });
    if !download_output.status.success() {
        panic!("\n{:?}\n{}\n{}\n",
               download_cmd,
               String::from_utf8_lossy(&download_output.stdout),
               String::from_utf8_lossy(&download_output.stderr));
    }

    // Unpack the tarball
    let gz_archive = unwrap!(File::open(&gz_path));
    let gz_decoder = unwrap!(GzDecoder::new(gz_archive));
    let mut archive = Archive::new(gz_decoder);

    // Extract just the appropriate version of libsodium.a and headers to the install path
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

    // Clean up
    let _ = fs::remove_file(gz_path);

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



#[cfg(all(not(windows), not(feature = "use-installed-libsodium")))]
fn main() {
    use std::env;
    use std::fs::{self, File};
    use std::process::Command;
    use flate2::read::GzDecoder;
    use tar::Archive;

    // Download gz tarball
    let basename = "libsodium-".to_string() + VERSION;
    let gz_filename = basename.clone() + ".tar.gz";
    let url = "https://github.com/jedisct1/libsodium/releases/download/".to_string() +
              VERSION + "/" + &gz_filename;
    let mut install_dir = get_install_dir();
    let mut source_dir = unwrap!(env::var("OUT_DIR")) + "/source";
    // Avoid issues with paths containing spaces by falling back to using /tmp
    let target = unwrap!(env::var("TARGET"));
    if install_dir.contains(" ") {
        let fallback_path = "/tmp/".to_string() + &basename + "/" + &target;
        install_dir = fallback_path.clone() + "/installed";
        source_dir = fallback_path.clone() + "/source";
        println!("cargo:warning=The path to the usual build directory contains spaces and hence \
                  can't be used to build libsodium.  Falling back to use {}.  If running `cargo \
                  clean`, ensure you also delete this fallback directory",
                 fallback_path);
    }
    let gz_path = source_dir.clone() + "/" + &gz_filename;
    unwrap!(fs::create_dir_all(&install_dir));
    unwrap!(fs::create_dir_all(&source_dir));

    let mut curl_cmd = Command::new("curl");
    let curl_output = curl_cmd
        .arg(&url)
        .arg("-sSLvo")
        .arg(&gz_path)
        .output()
        .unwrap_or_else(|error| {
                            panic!("Failed to run curl command: {}", error);
                        });
    if !curl_output.status.success() {
        panic!("\n{:?}\n{}\n{}\n",
               curl_cmd,
               String::from_utf8_lossy(&curl_output.stdout),
               String::from_utf8_lossy(&curl_output.stderr));
    }

    // Unpack the tarball
    let gz_archive = unwrap!(File::open(&gz_path));
    let gz_decoder = unwrap!(GzDecoder::new(gz_archive));
    let mut archive = Archive::new(gz_decoder);
    unwrap!(archive.unpack(&source_dir));
    source_dir.push_str(&format!("/{}", basename));

    // Clean up
    let _ = fs::remove_file(gz_path);

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

    // Disable PIE for Ubuntu < 15.04 (see https://github.com/jedisct1/libsodium/issues/292)
    let disable_pie_arg = match Command::new("lsb_release").arg("-irs").output() {
        Ok(id_output) => {
            let stdout = String::from_utf8_lossy(&id_output.stdout);
            let mut lines = stdout.lines();
            if lines.next() == Some("Ubuntu") {
                let v = unwrap!(unwrap!(unwrap!(lines.next()).split('.').next()).parse::<u32>());
                if v < 15 { "--disable-pie" } else { "" }
            } else {
                ""
            }
        }
        _ => "",
    };

    let mut configure_cmd = Command::new("./configure");
    let configure_output = configure_cmd
        .current_dir(&source_dir)
        .env("CC", &cc)
        .env("CFLAGS", &cflags)
        .arg(&prefix_arg)
        .arg(&host_arg)
        .arg("--enable-shared=no")
        .arg(disable_pie_arg)
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
