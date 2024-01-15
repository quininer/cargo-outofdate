mod query;

use std::path::Path;
use std::borrow::Cow;
use std::io::{ self, Write };
use cargo::ops;
use cargo::core::Workspace;
use cargo::core::registry::PackageRegistry;
use cargo::util::Config as CargoConfig;
use cargo::util::errors::CargoResult;
use cargo::util::cache_lock::CacheLockMode;
use cargo::util::important_paths::find_root_manifest_for_wd;
use tabwriter::TabWriter;
use argh::FromArgs;
use query::query_latest;


const EMPTY_VERSION: Cow<'static, str> = Cow::Borrowed("--");

/// cargo outofdate checker
#[derive(FromArgs)]
struct Options {
    /// manifest path
    #[argh(option, short = 'm')]
    manifest: Option<String>,

    /// only check root dependencies
    #[argh(switch, short = 'R')]
    only_root: bool,

    /// run without update crates-io
    #[argh(switch)]
    offline: bool,

    #[argh(positional)]
    _ignore: Option<String>,
}


#[inline]
fn start(options: Options) -> CargoResult<()> {
    let mut config = CargoConfig::default()?;
    config.configure(
        1,
        false,
        None,
        options.offline,
        true,
        options.offline,
        &None,
        &[],
        &[]
    )?;
    let _guard = config.acquire_package_cache_lock(CacheLockMode::DownloadExclusive)?;

    let workspace = {
        let manifest_path = if let Some(manifest) = options.manifest.as_ref() {
            Path::new(manifest).canonicalize()?
        } else {
            find_root_manifest_for_wd(config.cwd())?
        };
        Workspace::new(&manifest_path, &config)?
    };

    let mut registry = PackageRegistry::new(&config)?;
    registry.lock_patches();

    let (_, resolve) = ops::resolve_ws(&workspace)?;

    let packages = if options.only_root {
        Some(workspace
            .members()
            .collect::<Vec<_>>())
    } else {
        None
    };

    let mut results = Vec::new();

    for pkg in resolve.iter() {
        if let Some(packages) = packages.as_ref() {
            if !packages.iter()
                .map(|dep| dep.dependencies())
                .flatten()
                .any(|dep| dep.matches_id(pkg))
            {
                continue
            }
        }

        let (compat_latest, latest) = query_latest(&mut registry, &pkg)?;

        if compat_latest.is_some() || latest.is_some() {
            results.push((pkg, compat_latest, latest));
        }
    }

    if results.is_empty() {
        config.shell()
            .status("Ok", "All dependencies are up to date, yay!")?;
    } else {
        results.sort_by_key(|&(pkg, _, _)| pkg);

        let mut tw = TabWriter::new(Vec::new());
        writeln!(&mut tw, "Name\tNow\tCompat\tLatest")?;

        for (pkg, compat_latest, latest) in results {
            writeln!(&mut tw, "{}:\t{}\t{}\t{}",
                pkg.name(),
                pkg.version(),
                compat_latest
                    .map(|s| s.version().to_string())
                    .map(Cow::Owned)
                    .unwrap_or(EMPTY_VERSION),
                latest
                    .map(|s| s.version().to_string())
                    .map(Cow::Owned)
                    .unwrap_or(EMPTY_VERSION)
            )?;
        }

        io::stdout().write_all(&tw.into_inner()?)?;
    }

    Ok(())
}

fn main() {
    let options = argh::from_env();
    start(options).unwrap();
}
