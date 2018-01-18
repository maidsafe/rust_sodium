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
        println!(
            "cargo:warning=Using unknown libsodium version.  This crate is tested against \
                  {} and may not be fully compatible with other versions.",
            VERSION
        );
    } else {
        let lib_details = unwrap!(pkg_config::probe_library("libsodium"));
        if lib_details.version != VERSION {
            println!(
                "cargo:warning=Using libsodium version {}.  This crate is tested against {} \
                      and may not be fully compatible with {}.",
                lib_details.version,
                VERSION,
                lib_details.version
            );
        }
    }
}



#[cfg(all(not(windows), not(feature = "use-installed-libsodium")))]
extern crate gcc;
#[cfg(all(not(target_env = "msvc"), not(feature = "use-installed-libsodium")))]
extern crate flate2;
#[cfg(all(not(target_env = "msvc"), not(feature = "use-installed-libsodium")))]
extern crate tar;
#[cfg(all(target_env = "msvc", not(feature = "use-installed-libsodium")))]
extern crate libc;
#[cfg(all(target_env = "msvc", not(feature = "use-installed-libsodium")))]
extern crate zip;

#[cfg(not(feature = "use-installed-libsodium"))]
fn get_install_dir() -> String {
    use std::env;
    unwrap!(env::var("OUT_DIR")) + "/installed"
}

#[cfg(all(windows, not(feature = "use-installed-libsodium")))]
fn check_powershell_version() {
    let mut check_ps_version_cmd = ::std::process::Command::new("powershell");
    let check_ps_version_output = check_ps_version_cmd
        .arg("-Command")
        .arg("If ($PSVersionTable.PSVersion.Major -lt 4) { exit 1 }")
        .output()
        .unwrap_or_else(|error| {
            panic!("Failed to run powershell command: {}", error);
        });
    if !check_ps_version_output.status.success() {
        panic!(
            "\n{:?}\n{}\n{}\nYou must have Powershell v4.0 or greater installed.\n\n",
            check_ps_version_cmd,
            String::from_utf8_lossy(&check_ps_version_output.stdout),
            String::from_utf8_lossy(&check_ps_version_output.stderr)
        );
    }
}

#[cfg(all(windows, not(feature = "use-installed-libsodium")))]
fn download_compressed_file() -> String {
    use std::process::Command;

    let basename = "libsodium-".to_string() + VERSION;
    let zip_filename = if cfg!(target_env = "msvc") {
        basename.clone() + "-msvc.zip"
    } else {
        basename.clone() + "-mingw.tar.gz"
    };
    let url = "https://download.libsodium.org/libsodium/releases/".to_string() + &zip_filename;
    let zip_path = get_install_dir() + "/" + &zip_filename;
    let mut command = "([Net.ServicePointManager]::SecurityProtocol = 'Tls12') -and \
               ((New-Object System.Net.WebClient).DownloadFile(\""
        .to_string() + &url + "\", \"" + &zip_path + "\"))";
    let mut download_cmd = Command::new("powershell");
    let mut download_output = download_cmd
        .arg("-Command")
        .arg(&command)
        .output()
        .unwrap_or_else(|error| {
            panic!("Failed to run powershell download command: {}", error);
        });
    if download_output.status.success() {
        return zip_path;
    }

    let fallback_url = "https://raw.githubusercontent.com/maidsafe/QA/master/appveyor/"
        .to_string() + &zip_filename;
    println!(
        "cargo:warning=Failed to download libsodium from {}.  Falling back to MaidSafe mirror \
             at {}",
        url,
        fallback_url
    );
    command = "([Net.ServicePointManager]::SecurityProtocol = 'Tls12') -and \
               ((New-Object System.Net.WebClient).DownloadFile(\""
        .to_string() + &fallback_url + "\", \"" + &zip_path + "\"))";
    download_cmd = Command::new("powershell");
    download_output = download_cmd
        .arg("-Command")
        .arg(&command)
        .output()
        .unwrap_or_else(|error| {
            panic!("Failed to run powershell download command: {}", error);
        });
    if !download_output.status.success() {
        panic!(
            "\n{:?}\n{}\n{}\n",
            download_cmd,
            String::from_utf8_lossy(&download_output.stdout),
            String::from_utf8_lossy(&download_output.stderr)
        );
    }
    zip_path
}

#[cfg(all(windows, target_env = "msvc", not(feature = "use-installed-libsodium")))]
fn main() {
    use libc::S_IFDIR;
    use std::fs::{self, File};
    use std::io::{Read, Write};
    use std::path::Path;
    use zip::ZipArchive;

    check_powershell_version();

    // Download zip file
    let install_dir = get_install_dir();
    let lib_install_dir = Path::new(&install_dir).join("lib");
    unwrap!(fs::create_dir_all(&lib_install_dir));
    let zip_path = download_compressed_file();

    // Unpack the zip file
    let zip_file = unwrap!(File::open(&zip_path));
    let mut zip_archive = unwrap!(ZipArchive::new(zip_file));

    // Extract just the appropriate version of libsodium.lib and headers to the install path.  For
    // now, only handle MSVC 2015.
    let arch_path = if cfg!(target_pointer_width = "32") {
        Path::new("Win32")
    } else if cfg!(target_pointer_width = "64") {
        Path::new("x64")
    } else {
        panic!("target_pointer_width not 32 or 64")
    };

    let unpacked_lib = arch_path.join("Release/v140/static/libsodium.lib");
    for i in 0..zip_archive.len() {
        let mut entry = unwrap!(zip_archive.by_index(i));
        let entry_name = entry.name().to_string();
        let entry_path = Path::new(&entry_name);
        let opt_install_path = if entry_path.starts_with("include") {
            let is_dir = (unwrap!(entry.unix_mode()) & S_IFDIR as u32) != 0;
            if is_dir {
                let _ = fs::create_dir(&Path::new(&install_dir).join(entry_path));
                None
            } else {
                Some(Path::new(&install_dir).join(entry_path))
            }
        } else if entry_path == unpacked_lib {
            Some(lib_install_dir.join("libsodium.lib"))
        } else {
            None
        };
        if let Some(full_install_path) = opt_install_path {
            let mut buffer = Vec::with_capacity(entry.size() as usize);
            assert_eq!(entry.size(), unwrap!(entry.read_to_end(&mut buffer)) as u64);
            let mut file = unwrap!(File::create(&full_install_path));
            unwrap!(file.write_all(&buffer));
        }
    }

    // Clean up
    let _ = fs::remove_file(zip_path);

    println!("cargo:rustc-link-lib=static=libsodium");
    println!(
        "cargo:rustc-link-search=native={}",
        lib_install_dir.display()
    );
    println!("cargo:include={}/include", install_dir);
}



#[cfg(all(windows, not(target_env = "msvc"), not(feature = "use-installed-libsodium")))]
fn main() {
    use std::fs::{self, File};
    use std::path::Path;
    use flate2::read::GzDecoder;
    use tar::Archive;

    check_powershell_version();

    // Download gz tarball
    let install_dir = get_install_dir();
    let lib_install_dir = Path::new(&install_dir).join("lib");
    unwrap!(fs::create_dir_all(&lib_install_dir));
    let gz_path = download_compressed_file();

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
            lib_install_dir.join("libsodium.a")
        } else {
            continue;
        };
        unwrap!(entry.unpack(full_install_path));
    }

    // Clean up
    let _ = fs::remove_file(gz_path);

    println!("cargo:rustc-link-lib=static=sodium");
    println!(
        "cargo:rustc-link-search=native={}",
        lib_install_dir.display()
    );
    println!("cargo:include={}/include", install_dir);
}



/// Fetch and unpack the libsodium sources.
#[cfg(all(not(windows), not(feature = "use-installed-libsodium")))]
fn get_sources() -> (String, String) {
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
        println!(
            "cargo:warning=The path to the usual build directory contains spaces and hence \
                  can't be used to build libsodium.  Falling back to use {}.  If running `cargo \
                  clean`, ensure you also delete this fallback directory",
            fallback_path
        );
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
        panic!(
            "\n{:?}\n{}\n{}\n",
            curl_cmd,
            String::from_utf8_lossy(&curl_output.stdout),
            String::from_utf8_lossy(&curl_output.stderr)
        );
    }

    // Unpack the tarball
    let gz_archive = unwrap!(File::open(&gz_path));
    let gz_decoder = unwrap!(GzDecoder::new(gz_archive));
    let mut archive = Archive::new(gz_decoder);
    unwrap!(archive.unpack(&source_dir));
    source_dir.push_str(&format!("/{}", basename));

    // Clean up
    let _ = fs::remove_file(gz_path);

    (source_dir, install_dir)
}



#[cfg(all(not(windows), not(feature = "use-installed-libsodium")))]
fn main() {
    use std::env;
    use std::str;
    use std::path::Path;
    use std::process::Command;

    // Determine build target triple
    let target = unwrap!(env::var("TARGET"));

    // Download sources
    let (source_dir, install_dir) = get_sources();

    // Decide on CC, CFLAGS and the --host configure argument
    let gcc = gcc::Build::new();
    let mut cc = unwrap!(gcc.get_compiler().path().to_str()).to_string();
    let mut cflags = env::var("CFLAGS").unwrap_or(String::default());
    cflags += " -O2";
    let host_arg;
    let cross_compiling;
    let help;
    if target.contains("-ios") {
        // Determine Xcode directory path
        let xcode_select_output = unwrap!(Command::new("xcode-select").arg("-p").output());
        if !xcode_select_output.status.success() {
            panic!("Failed to run xcode-select -p");
        }
        let xcode_dir = unwrap!(str::from_utf8(&xcode_select_output.stdout))
            .trim()
            .to_string();

        // Determine SDK directory paths
        let sdk_dir_simulator = unwrap!(
            Path::new(&xcode_dir)
                .join(
                    "Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk",
                )
                .to_str()
        ).to_string();
        let sdk_dir_ios = unwrap!(
            Path::new(&xcode_dir)
                .join("Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk")
                .to_str()
        ).to_string();

        // Min versions
        let ios_simulator_version_min = "6.0.0";
        let ios_version_min = "6.0.0";

        // Roughly based on `dist-build/ios.sh` in the libsodium sources
        match &*target {
            "aarch64-apple-ios" => {
                cflags += " -arch arm64";
                cflags += &format!(" -isysroot {}", sdk_dir_ios);
                cflags += &format!(" -mios-version-min={}", ios_version_min);
                cflags += " -fembed-bitcode";
                host_arg = "--host=arm-apple-darwin10".to_string();
            }
            "armv7-apple-ios" => {
                cflags += " -arch armv7";
                cflags += &format!(" -isysroot {}", sdk_dir_ios);
                cflags += &format!(" -mios-version-min={}", ios_version_min);
                cflags += " -mthumb";
                host_arg = "--host=arm-apple-darwin10".to_string();
            }
            "armv7s-apple-ios" => {
                cflags += " -arch armv7s";
                cflags += &format!(" -isysroot {}", sdk_dir_ios);
                cflags += &format!(" -mios-version-min={}", ios_version_min);
                cflags += " -mthumb";
                host_arg = "--host=arm-apple-darwin10".to_string();
            }
            "i386-apple-ios" => {
                cflags += " -arch i386";
                cflags += &format!(" -isysroot {}", sdk_dir_simulator);
                cflags += &format!(" -mios-simulator-version-min={}", ios_simulator_version_min);
                host_arg = "--host=i686-apple-darwin10".to_string();
            }
            "x86_64-apple-ios" => {
                cflags += " -arch x86_64";
                cflags += &format!(" -isysroot {}", sdk_dir_simulator);
                cflags += &format!(" -mios-simulator-version-min={}", ios_simulator_version_min);
                host_arg = "--host=x86_64-apple-darwin10".to_string();
            }
            _ => panic!("Unknown iOS build target: {}", target),
        }
        cross_compiling = true;
        help = "";
    } else {
        if target.contains("i686") {
            cc += " -m32";
            cflags += " -march=i686";
        }
        let host = unwrap!(env::var("HOST"));
        host_arg = format!("--host={}", target);
        cross_compiling = target != host;
        help = if cross_compiling {
            "***********************************************************\n\
             Possible missing dependencies.\n\
             See https://github.com/maidsafe/rust_sodium#cross-compiling\n\
             ***********************************************************\n\n"
        } else {
            ""
        };
    }

    // Run `./configure`
    let prefix_arg = format!("--prefix={}", install_dir);
    let mut configure_cmd = Command::new("./configure");
    if !cc.is_empty() {
        configure_cmd.env("CC", &cc);
    }
    if !cflags.is_empty() {
        configure_cmd.env("CFLAGS", &cflags);
    }
    if env::var("RUST_SODIUM_DISABLE_PIE").is_ok() {
        configure_cmd.arg("--disable-pie");
    }
    let configure_output = configure_cmd
        .current_dir(&source_dir)
        .arg(&prefix_arg)
        .arg(&host_arg)
        .arg("--enable-shared=no")
        .output()
        .unwrap_or_else(|error| {
            panic!("Failed to run './configure': {}\n{}", error, help);
        });
    if !configure_output.status.success() {
        panic!(
            "\n{:?}\nCFLAGS={}\nCC={}\n{}\n{}\n{}\n",
            configure_cmd,
            cflags,
            cc,
            String::from_utf8_lossy(&configure_output.stdout),
            String::from_utf8_lossy(&configure_output.stderr),
            help
        );
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
        panic!(
            "\n{:?}\n{}\n{}\n{}\n{}",
            make_cmd,
            String::from_utf8_lossy(&configure_output.stdout),
            String::from_utf8_lossy(&make_output.stdout),
            String::from_utf8_lossy(&make_output.stderr),
            help
        );
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
        panic!(
            "\n{:?}\n{}\n{}\n{}\n{}\n",
            install_cmd,
            String::from_utf8_lossy(&configure_output.stdout),
            String::from_utf8_lossy(&make_output.stdout),
            String::from_utf8_lossy(&install_output.stdout),
            String::from_utf8_lossy(&install_output.stderr)
        );
    }

    println!("cargo:rustc-link-lib=static=sodium");
    println!("cargo:rustc-link-search=native={}/lib", install_dir);
    println!("cargo:include={}/include", install_dir);
}
