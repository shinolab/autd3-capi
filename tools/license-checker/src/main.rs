use std::path::Path;

use cargo_license::{get_dependencies_from_cargo_lock, GetDependenciesOpt};
use cargo_metadata::MetadataCommand;

const WHITE_LIST: &[&str] = &[
    "autd3",
    "autd3-core",
    "autd3-derive",
    "autd3-driver",
    "autd3-firmware-emulator",
    "autd3-gain-holo",
    "autd3-link-remote",
    "autd3-link-twincat",
    "autd3capi",
    "autd3capi-driver",
    "autd3capi-gain-holo",
    "autd3capi-link-remote",
    "autd3capi-link-twincat",
];

fn main() -> anyhow::Result<()> {
    let mut cmd = MetadataCommand::new();
    cmd.manifest_path(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml"));

    let get_opts = GetDependenciesOpt {
        avoid_dev_deps: true,
        avoid_build_deps: true,
        direct_deps_only: false,
        avoid_proc_macros: false,
        root_only: false,
    };

    let dependencies = get_dependencies_from_cargo_lock(&cmd, &get_opts)?;

    let dependencies: Vec<_> = dependencies
        .into_iter()
        .filter(|dep| !WHITE_LIST.contains(&dep.name.as_str()))
        .collect();

    if dependencies.is_empty() {
        return Ok(());
    }

    eprintln!("The following dependencies have found.");
    for dep in dependencies {
        eprintln!(
            "\t{} {} {}",
            dep.name,
            dep.version,
            dep.license.unwrap_or_else(|| "UNKNOWN".to_string())
        );
    }

    Err(anyhow::anyhow!("Please check the dependencies."))
}
