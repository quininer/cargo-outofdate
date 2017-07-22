use cargo::core::registry::{ PackageRegistry, Registry };
use cargo::core::dependency::Dependency;
use cargo::core::package_id::PackageId;
use cargo::core::summary::Summary;
use cargo::util::errors::CargoResult;
use semver::VersionReq;


pub fn query_latest(registry: &mut PackageRegistry, package: &PackageId)
    -> CargoResult<(Option<Summary>, Option<Summary>)>
{
    let dep = Dependency::new_override(package.name(), package.source_id());
    let results = registry.query_vec(&dep)?;
    let package_version = VersionReq::parse(&package.version().to_string())?;

    let compatible_latest = results.iter()
        .filter(|summary| package_version.matches(summary.version()))
        .max_by_key(|summary| summary.version())
        .and_then(|summary|
            if summary.version() > package.version() { Some(summary) }
            else { None }
        )
        .cloned();
    let latest = results.iter()
        .max_by_key(|summary| summary.version())
        .and_then(|summary|
            if summary.version() > package.version() { Some(summary) }
            else { None }
        )
        .and_then(|summary|
            if Some(summary) == compatible_latest.as_ref() { None }
            else { Some(summary) }
        )
        .cloned();

    Ok((compatible_latest, latest))
}
