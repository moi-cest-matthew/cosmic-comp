// SPDX-License-Identifier: GPL-3.0-only
extern crate wayland_scanner;

use std::{env, path::PathBuf, process::Command};
use wayland_scanner::{generate_code, Side};

fn main() {
    if let Some(output) = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .ok()
    {
        let git_hash = String::from_utf8(output.stdout).unwrap();
        println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    }

    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());
    // Location of the xml file, relative to the `Cargo.toml`
    let drm_protocol_file = "resources/wayland-drm.xml";
    let ext_workspace_protocol_file = "resources/ext-workspace-unstable-v1.xml";
    // Target directory for the generate files
    generate_code(drm_protocol_file, &dest.join("wl_drm.rs"), Side::Server);
    generate_code(ext_workspace_protocol_file, &dest.join("ext_workspace.rs"), Side::Server);
}
