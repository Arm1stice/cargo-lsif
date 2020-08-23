// SPDX-FileCopyrightText: 2020 Wyatt Calandro <arm1stice@arm1stice.com>
//
// SPDX-License-Identifier: Apache-2.0

#![feature(rustc_private)]

mod save_analysis;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Crate {
    pub package: Package,
}

// Test function
pub fn generate_lsif(toml_contents: String) -> Result<(), toml::de::Error> {
    // Attempt to retrieve information about the crate from the contents of the Cargo.toml file
    let krate: Crate = toml::from_str(&toml_contents)?;
    let krate_name = krate.package.name;
    let krate_version = krate.package.version;

    Ok(())
}
