use anyhow::Context;
use std::path::{Path, PathBuf};

use wit_parser::{PackageId, Resolve};

fn main() -> anyhow::Result<()> {
    let (mut resolve, _) = parse_wit(&PathBuf::from("./wit"))?;
    let (world_one, _) = resolve
        .worlds
        .iter()
        .find(|(world, _)| resolve.worlds[*world].name == "one")
        .unwrap();
    let (world_two, _) = resolve
        .worlds
        .iter()
        .find(|(world, _)| resolve.worlds[*world].name == "two")
        .unwrap();
    resolve
        .merge_worlds(world_two, world_one)
        .with_context(|| "failed to merge worlds")?;

    Ok(())
}

fn parse_wit(path: &Path) -> anyhow::Result<(Resolve, Vec<PackageId>)> {
    let mut resolve = Resolve::default();
    let id = if path.is_dir() {
        resolve.push_dir(&path)?.0
    } else {
        resolve.push_file(&path)?
    };
    Ok((resolve, id))
}
