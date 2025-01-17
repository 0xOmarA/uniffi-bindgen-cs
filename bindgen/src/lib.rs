/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub mod gen_cs;
use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use fs_err::File;
pub use gen_cs::{generate_bindings, Config};
use std::io::Write;
use uniffi_bindgen::interface::ComponentInterface;

#[derive(Parser)]
#[clap(name = "uniffi-bindgen")]
#[clap(version = clap::crate_version!())]
#[clap(propagate_version = true)]
struct Cli {
    /// Directory in which to write generated files. Default is same folder as .udl file.
    #[clap(long, short)]
    out_dir: Option<Utf8PathBuf>,

    /// Do not try to format the generated bindings.
    #[clap(long, short)]
    no_format: bool,

    /// Extract proc-macro metadata from a native lib (cdylib or staticlib) for this crate.
    #[clap(long)]
    lib_file: Option<Utf8PathBuf>,

    /// Path to the optional uniffi config file. If not provided, uniffi-bindgen will try to guess it from the UDL's file location.
    #[clap(long, short)]
    config: Option<Utf8PathBuf>,

    /// Path to the UDL file.
    udl_file: Utf8PathBuf,
}

struct BindingGeneratorCs {
    _try_format_code: bool,
}

impl uniffi_bindgen::BindingGenerator for BindingGeneratorCs {
    type Config = gen_cs::Config;

    fn write_bindings(
        &self,
        ci: ComponentInterface,
        config: Self::Config,
        out_dir: &Utf8Path,
    ) -> anyhow::Result<()> {
        let bindings_file = out_dir.join(format!("{}.cs", ci.namespace()));
        let mut f = File::create(&bindings_file)?;
        write!(f, "{}", generate_bindings(&config, &ci)?)?;

        // TODO: find a way to easily format standalone C# files
        // https://github.com/dotnet/format

        Ok(())
    }
}

impl uniffi_bindgen::BindingsConfig for gen_cs::Config {
    const TOML_KEY: &'static str = "csharp";

    fn update_from_cdylib_name(&mut self, cdylib_name: &str) {
        self.cdylib_name
            .get_or_insert_with(|| cdylib_name.to_string());
    }

    fn update_from_ci(&mut self, _ci: &ComponentInterface) {}

    fn update_from_dependency_configs(
        &mut self,
        _config_map: std::collections::HashMap<&str, &Self>,
    ) {
    }
}

pub fn main() {
    let cli = Cli::parse();
    uniffi_bindgen::generate_external_bindings(
        BindingGeneratorCs {
            _try_format_code: !cli.no_format,
        },
        &cli.udl_file,
        cli.config,
        cli.out_dir,
        cli.lib_file.as_deref(),
    )
    .unwrap();
}
