// smbclient-sys (rust bindings for libsmbclient from Samba project)
// Copyright (c) 2016 Konstantin Gribov
//
// This file is part of smbclient-sys.
//
// smbclient-sys is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// smbclient-sys is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with smbclient-sys.  If not, see <http://www.gnu.org/licenses/>.

extern crate libbindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let lib = pkg_config::Config::new()
        .atleast_version("4.0.0")
        .probe("smbclient")
        .expect("libsmbclient v4.0.0+ not found");

    let header = find_header(&lib).unwrap().to_str().unwrap().to_string();

    let bindings = libbindgen::Builder::default()
        .no_unstable_rust()
        .header(header)
        .ctypes_prefix("::libc")
        .link("smbclient")
        .whitelisted_type("(smbc_.*)|(SMBC.*)")
        .whitelisted_function("smbc_.*")
        .whitelisted_var("SMBC?.*")
        .generate()
        .expect("failed to generate bindings for libsmbclient");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_dir.join("libsmbclient.rs"))
            .expect("failed to write libsmbclient.rs");
}

fn find_header(lib: &pkg_config::Library) -> Option<PathBuf> {
    lib.include_paths.iter()
                     .map(|p| p.join("libsmbclient.h"))
                     .find(|p| p.exists() && p.is_file())
}
