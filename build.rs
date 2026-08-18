use vergen_gitcl::{BuildBuilder, CargoBuilder, Emitter, GitclBuilder, RustcBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build = BuildBuilder::default().build_timestamp(true).build()?;
    let cargo = CargoBuilder::default().target_triple(true).build()?;
    let gitcl = GitclBuilder::default().sha(true).dirty(true).build()?;
    let rustc = RustcBuilder::default().semver(true).build()?;

    let mut emitter = Emitter::default();
    // Offloaded builds can temporarily miss git objects and trigger fallback warnings.
    // Keep default env fallbacks, but suppress warning noise in build output.
    emitter
        .quiet()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&gitcl)?
        .add_instructions(&rustc)?
        .emit()?;

    Ok(())
}
