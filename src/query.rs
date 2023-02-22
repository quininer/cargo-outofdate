use std::task::Poll;
use cargo::core::registry::{ PackageRegistry, Registry };
use cargo::core::dependency::Dependency;
use cargo::core::package_id::PackageId;
use cargo::core::summary::Summary;
use cargo::core::source::QueryKind;
use cargo::util::errors::CargoResult;
use semver::VersionReq;


pub fn query_latest(registry: &mut PackageRegistry, package: &PackageId)
    -> CargoResult<(Option<Summary>, Option<Summary>)>
{
    let dep = Dependency::new_override(package.name(), package.source_id());
    let results = loop {
        match registry.query_vec(&dep, QueryKind::Exact)? {
            Poll::Ready(results) => break results,
            Poll::Pending => registry.block_until_ready()?
        }
    };
    let package_version = VersionReq::parse(&package.version().to_string())?;

    let compatible_latest = results.iter()
        .filter(|summary| package_version.matches(summary.version()))
        .max_by_key(|summary| summary.version())
        .filter(|summary| summary.version() > package.version())
        .cloned();
    let latest = results.iter()
        .max_by_key(|summary| summary.version())
        .filter(|summary| summary.version() > package.version())
        .filter(|summary| Some(*summary) != compatible_latest.as_ref())
        .cloned();

    Ok((compatible_latest, latest))
}
