// SPDX-FileCopyrightText: 2020 Wyatt Calandro <arm1stice@arm1stice.com>
//
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate anyhow;

extern crate cargo_lsif;

use anyhow::{Context, Result};
use std::env;

fn main() -> Result<()> {
    let cwd = env::current_dir().context("Failed to get current working directory")?;
    let toml_path = cwd.join("Cargo.toml");

    if cwd.join("Cargo.toml").exists() {
        // Read the contents of the Cargo.toml file
        let contents =
            std::fs::read_to_string(toml_path).context("Failed to read the Cargo.toml file")?;

        // Attempt to generate the lsif data
        cargo_lsif::generate_lsif(contents).context("Failed to generate lsif data")?;
    } else {
        anyhow!("Missing Cargo.toml file in the current directory");
    }

    Ok(())
}
