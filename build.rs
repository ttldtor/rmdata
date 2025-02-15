// Copyright (c) 2025 ttldtor.
// SPDX-License-Identifier: BSL-1.0

use reqwest;
use std::io::Cursor;
use std::{env, fs, path::Path};
use toml::Value;
use zip::ZipArchive;

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");
    let cargo_toml = fs::read_to_string(cargo_toml_path).expect("Unable to read Cargo.toml");
    let parsed_cargo_toml: Value = cargo_toml.parse().expect("Unable to parse Cargo.toml");

    let graal_native_sdk = &parsed_cargo_toml["package"]["metadata"]["dxfeed"]["graalnativesdk"];
    let lib_name = graal_native_sdk["lib-name"].as_str().unwrap();
    let sdk_version = graal_native_sdk["version"].as_str().unwrap();
    let sdk_url_template = graal_native_sdk["url-template"].as_str().unwrap();

    let target = env::var("TARGET").unwrap();

    println!("cargo::warning=sdk ver = {:?}", sdk_version);
    println!("cargo::warning=sdk url template = {:?}", sdk_url_template);
    println!("cargo::warning=target = {:?}", target);

    #[cfg(target_os = "linux")]
    let normalized_os = "linux";

    #[cfg(target_os = "windows")]
    let normalized_os = "windows";

    #[cfg(target_os = "macos")]
    let normalized_os = "osx";

    #[cfg(target_os = "ios")]
    let normalized_os = "ios";

    #[cfg(not(any(
        target_os = "linux",
        target_os = "windows",
        target_os = "macos",
        target_os = "ios"
    )))]
    panic!("Unsupported OS");

    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    let normalized_cpu = "aarch64";

    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    let normalized_cpu = "amd64";

    #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
    let normalized_cpu = "amd64";

    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    let normalized_cpu = "aarch64";

    #[cfg(all(target_arch = "x86_64", target_os = "macos"))]
    let normalized_cpu = "x86_64";

    #[cfg(all(target_arch = "aarch64", target_os = "ios"))]
    let normalized_cpu = "aarch64";

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    panic!("Unsupported CPU");

    let sdk_url = sdk_url_template
        .replace("{version}", sdk_version)
        .replace("{cpu}", normalized_cpu)
        .replace("{os}", normalized_os);

    println!("cargo::warning=sdk url = {:?}", sdk_url);

    let response = reqwest::get(sdk_url).await.expect("Unable to download SDK");
    let mut archive = ZipArchive::new(Cursor::new(
        response.bytes().await.expect("Unable to read archive"),
    ))
    .expect("Unable to open archive");

    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_dir = Path::new(&out_dir).join("c_lib");

    fs::create_dir_all(&lib_dir).unwrap();

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .expect("Failed to access file in archive");
        let out_path = lib_dir.join(file.name());
        if file.is_dir() {
            fs::create_dir_all(&out_path).unwrap();
        } else {
            let mut outfile = fs::File::create(&out_path).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    #[cfg(target_os = "ios")]
    {
        println!("cargo:rustc-link-lib=static={}", lib_name);
    }

    #[cfg(not(target_os = "ios"))]
    {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);

        #[cfg(target_os = "windows")]
        {
            let dll_name = format!("{}.dll", lib_name);
            let from_dir = lib_dir.join(dll_name.as_str());
            //let build_type = env::var("PROFILE").unwrap();
            //<root or manifest path>/target/<profile>/
            //let to_dir = Path::new(&manifest_dir).join("target").join(build_type);
            //let to_dir = Path::new(&manifest_dir).join(dll_name.as_str());
            let to_dir = Path::new(&manifest_dir).join("target").join("debug").join("deps");
            //let to_dir = Path::new(&out_dir).join(dll_name.as_str());

            fs::copy(from_dir, to_dir).unwrap();
        }
    }
}
