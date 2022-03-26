use std::{env, path::Path};

struct SlangReleaseInfo {
    url: &'static str,
    relative_path_to_binaries: &'static str,
    static_libs: &'static [&'static str],
}
macro_rules! slang_url {
    ($platform:literal) => {
        concat!(
            "https://github.com/shader-slang/slang/releases/download/v",
            include_str!("version.txt"),
            "/slang-",
            include_str!("version.txt"),
            "-",
            $platform,
            ".zip"
        )
    };
}
#[cfg(all(target_os = "windows", target_arch = "x86"))]
const SLANG_RELEASE: SlangReleaseInfo = SlangReleaseInfo {
    url: slang_url!("win32"),
    relative_path_to_binaries: "bin/windows-x86/release/",
    static_libs: &["slang"],
};
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const SLANG_RELEASE: SlangReleaseInfo = SlangReleaseInfo {
    url: slang_url!("win64"),
    relative_path_to_binaries: "bin/windows-x64/release/",
    static_libs: &["slang"],
};
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const SLANG_RELEASE: SlangReleaseInfo = SlangReleaseInfo {
    url: slang_url!("linux-x86_64"),
    relative_path_to_binaries: "bin/linux-x64/release/",
    static_libs: &[],
};
#[cfg(not(any(
    all(target_os = "linux", target_arch = "x86_64"),
    all(
        target_os = "windows",
        any(target_arch = "x86", target_arch = "x86_64")
    )
)))]
compile_error!("No official release for the current platform!");

fn main() {
    let filename = SLANG_RELEASE.url.split("/").last().unwrap();

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(filename);
    if !path.exists() {
        // Download release file
        #[cfg(target_os = "windows")]
        std::process::Command::new("powershell")
            .arg("Invoke-WebRequest")
            .arg(SLANG_RELEASE.url)
            .arg("-OutFile")
            .arg(path.as_os_str())
            .status()
            .unwrap();

        #[cfg(target_os = "linux")]
        std::process::Command::new("curl")
            .arg("--location")
            .arg("-s")
            .arg("-o")
            .arg(path.as_os_str())
            .arg(SLANG_RELEASE.url)
            .status()
            .unwrap();
    }
    let dir_name = filename.replace(".zip", "");

    let unzipped_path = Path::new(&env::var("OUT_DIR").unwrap()).join(dir_name);
    if !unzipped_path.exists() {
        // Unzip release file
        #[cfg(target_os = "windows")]
        std::process::Command::new("powershell")
            .arg("Expand-Archive")
            .arg("-Force")
            .arg("-Path")
            .arg(path.as_os_str())
            .arg("-DestinationPath")
            .arg(unzipped_path.as_os_str())
            .status()
            .unwrap();

        #[cfg(target_os = "linux")]
        std::process::Command::new("unzip")
            .arg("-q")
            .arg(path.as_os_str())
            .arg("-d")
            .arg(unzipped_path.as_os_str())
            .status()
            .unwrap();
        // Remove zip file to save space
        std::fs::remove_file(path).unwrap();
    }

    // emit cargo metadata
    {
        let slang_binaries_path = unzipped_path.join(SLANG_RELEASE.relative_path_to_binaries);
        println!("cargo:rustc-link-search={}", slang_binaries_path.display());

        for static_lib in SLANG_RELEASE.static_libs {
            println!("cargo:rustc-link-lib=static={}", static_lib);
        }
    }
}
