extern crate cheddar;

fn main() {
    // Generate mp4parse.h.
    cheddar::Cheddar::new().expect("could not read manifest")
        .module("capi").expect("invalid module path")
        .insert_code("// This Source Code Form is subject to the terms of the Mozilla Public\n")
        .insert_code("// License, v. 2.0. If a copy of the MPL was not distributed with this\n")
        .insert_code("// file, You can obtain one at https://mozilla.org/MPL/2.0/.")
        .run_build("include/mp4parse.h");

    // Try compiling it.
    let job = std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg("make -C examples")
        .status()
        .expect("error: Unable to invoke makefile.");
    assert!(job.success(), "error: C++ test build failed.");
}