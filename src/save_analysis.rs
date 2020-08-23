// SPDX-FileCopyrightText: 2020 Wyatt Calandro <arm1stice@arm1stice.com>
//
// SPDX-License-Identifier: Apache-2.0

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_save_analysis;
extern crate rustc_session;

use rustc_driver::run_compiler;
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::{interface, Queries};
use rustc_save_analysis::DumpHandler;
use std::path::PathBuf;

// The following is code that I originally wrote for https://github.com/kythe/kythe

/// Handles compiler callbacks to enable and dump the save_analysis
#[derive(Default)]
struct CallbackShim {
    output_dir: PathBuf,
}

impl CallbackShim {
    /// Create a new CallbackShim that dumps save_analysis files to `output_dir`
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }
}

impl Callbacks for CallbackShim {
    // Always enable save_analysis generation
    fn config(&mut self, config: &mut interface::Config) {
        config.opts.debugging_opts.save_analysis = true;
    }

    fn after_analysis<'tcx>(
        &mut self,
        compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        let input = compiler.input();
        let crate_name = queries.crate_name().unwrap().peek().clone();

        // Configure the save_analysis to include full documentation.
        // Normally this would be set using a `rls_data::config::Config` struct on the
        // fourth parameter of `process_crate`. However, the Rust compiler
        // falsely claims that there is a mismatch between rustc_save_analysis's
        // `rls_data::config::Config` and ours, even though we use the same version.
        // This forces us to use the environment variable method of configuration
        // instead.
        std::env::set_var(
            "RUST_SAVE_ANALYSIS_CONFIG",
            r#"{"output_file":null,"full_docs":true,"pub_only":false,"reachable_only":false,"distro_crate":false,"signatures":false,"borrow_data":false}"#,
        );

        // Perform the save_analysis and dump it to the directory
        // The JSON file is saved at {self.output_dir}/save-analysis/{crate_name}.json
        queries.global_ctxt().unwrap().peek_mut().enter(|tcx| {
            rustc_save_analysis::process_crate(
                tcx,
                &crate_name,
                &input,
                None,
                DumpHandler::new(Some(self.output_dir.as_path()), &crate_name),
            )
        });

        Compilation::Continue
    }
}
