use anyhow::Result;
use std::env;
use vergen_gitcl::{
    BuildBuilder, CargoBuilder, Emitter, GitclBuilder, RustcBuilder, SysinfoBuilder,
};

fn main() -> Result<()> {
    let build = BuildBuilder::all_build()?;
    let cargo = CargoBuilder::all_cargo()?;
    let git = GitclBuilder::all_git()?;
    let rustc = RustcBuilder::all_rustc()?;
    let si = SysinfoBuilder::all_sysinfo()?;

    // Prepare the emitter and add all standard vergen instructions
    let mut emitter = Emitter::default();
    emitter
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&git)?
        .add_instructions(&rustc)?
        .add_instructions(&si)?;

    // ✅ Manually add Cargo build profile (debug/release)
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-env=CARGO_PROFILE={}", profile);
    }

    // Emit all vergen-generated and custom variables
    emitter.emit()?;
    Ok(())
}
