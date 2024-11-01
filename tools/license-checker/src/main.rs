use std::path::Path;

use autd3_license_check::license_file::{self};

fn main() -> anyhow::Result<()> {
    let license_file_map = license_file::load_license_file_map(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("license-file.toml"),
    )?;

    let changed = autd3_license_check::check(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml"),
        "ThirdPartyNotice",
        &license_file_map,
        &[("ADS", "ADS 0.0.25-1 (MIT)\nhttps://github.com/Beckhoff/ADS")],
    )?;

    if changed {
        return Err(anyhow::anyhow!(
            "Some ThirdPartyNotice.txt files have been updated. Manuall check is required.",
        ));
    }

    Ok(())
}
