fn main() {
    #[cfg(target_os = "windows")]
    windows::link_vlc();
}

#[cfg(target_os = "windows")]
mod windows {
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    compile_error!("Only x86 and x86_64 are supported at the moment. Adding support for other architectures should be trivial.");

    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::str::FromStr;

    use vswhom::VsFindResult;

    pub fn link_vlc() {
        println!("cargo:rerun-if-changed=.env");

        dotenv::dotenv().ok();

        // somehow it only works when using a path with double slashes like: C:\\Users\\...\\mpv
        let mpv_dir_str = env::var("MPV_DIR").expect("No MPV_DIR found in .env file");
        let mpv_path = PathBuf::from_str(mpv_dir_str.as_str()).unwrap();

        let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

        let vs = VsFindResult::search().expect("Could not locate Visual Studio");
        let vs_exe_path = PathBuf::from(
            vs.vs_exe_path
                .expect("Could not retrieve executable path for Visual Studio"),
        );

        generate_lib_from_dll(&out_dir, &vs_exe_path, &mpv_path);
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        // NOTE: Without this directive, linking fails with:
        //       ```
        //       error LNK2019: unresolved external symbol vsnprintf referenced in function _{MangledSymbolName}
        //          msvcrt.lib(vsnprintf.obj) : error LNK2001: unresolved external symbol vsnprintf
        //          msvcrt.lib(vsnprintf.obj) : error LNK2001: unresolved external symbol _vsnprintf
        //       ```
        //       https://stackoverflow.com/a/34230122
        println!("cargo:rustc-link-lib=dylib=legacy_stdio_definitions");
    }

    fn generate_lib_from_dll(out_dir: &Path, vs_exe_path: &Path, mpv_path: &Path) {
        // https://wiki.videolan.org/GenerateLibFromDll/

        let vs_dumpbin = vs_exe_path.join("dumpbin.exe");
        let vs_lib = vs_exe_path.join("lib.exe");
        let mpv_def_path = out_dir.join("mpv.def");
        let mpv_import_lib = out_dir.join("mpv.lib");

        let libmpv = mpv_path.join("mpv-2.dll");
        let exports = Command::new(vs_dumpbin)
            .current_dir(out_dir)
            .arg("/EXPORTS")
            .arg(libmpv.display().to_string().trim_end_matches(r"\"))
            .output()
            .unwrap();
        let exports = String::from_utf8(exports.stdout).unwrap();

        let mut mpv_def = String::from("EXPORTS\n");
        for line in exports.lines() {
            if let Some(line) = line.get(26..) {
                if line.starts_with("mpv_") {
                    mpv_def.push_str(line);
                    mpv_def.push_str("\r\n");
                }
            }
        }
        fs::write(&mpv_def_path, mpv_def.into_bytes()).unwrap();

        // FIXME: Handle paths with spaces in them.
        Command::new(vs_lib)
            .current_dir(out_dir)
            .arg("/NOLOGO")
            .args(&[
                format!(
                    r#"/DEF:{}"#,
                    mpv_def_path.display().to_string().trim_end_matches(r"\")
                ),
                format!(
                    r#"/OUT:{}"#,
                    mpv_import_lib.display().to_string().trim_end_matches(r"\")
                ),
                format!(
                    "/MACHINE:{}",
                    match target_arch().as_str() {
                        "x86" => "x86",
                        "x86_64" => "x64",
                        _ => unreachable!(),
                    }
                ),
            ])
            .spawn()
            .unwrap();
    }

    fn target_arch() -> String {
        env::var("CARGO_CFG_TARGET_ARCH").unwrap()
    }
}
