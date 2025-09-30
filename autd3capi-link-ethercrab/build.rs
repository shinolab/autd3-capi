#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;

    let out_dir = env::var("OUT_DIR")?;

    let tool = cc::windows_registry::find_tool(&env::var("TARGET")?, "lib.exe")
        .ok_or("lib.exe not found. Please install MSVC Build Tools.")?;

    tool.to_command()
        .arg("/MACHINE:X64")
        .arg("/DEF:def\\wpcapext.def")
        .arg(format!("/OUT:{}\\wpcapext.lib", out_dir))
        .output()?;

    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=wpcapext");

    Ok(())
}
