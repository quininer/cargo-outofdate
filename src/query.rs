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
    registry.block_until_ready()?;
    let results = match registry.query_vec(&dep, QueryKind::Exact)? {
        Poll::Ready(results) => results,
        Poll::Pending => anyhow::bail!("registry not ready")
    };
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
