#[macro_use] extern crate structopt_derive;
extern crate structopt;

extern crate cargo;
extern crate semver;
extern crate tabwriter;

mod query;

use std::path::Path;
use std::borrow::Cow;
use std::io::{ self, Write };
use cargo::ops;
use cargo::core::Workspace;
use cargo::core::registry::PackageRegistry;
use cargo::util::Config as CargoConfig;
use cargo::util::errors::CargoResult;
use cargo::util::important_paths::find_root_manifest_for_wd;
use tabwriter::TabWriter;
use structopt::StructOpt;
use query::query_latest;


#[derive(StructOpt)]
#[structopt]
pub struct Config {
    /// manifest path
    #[structopt(short = "m", long = "manifest")]
    manifest: Option<String>
}


#[inline]
fn start(config: Config) -> CargoResult<()> {
    const EMPTY_VERSION: Cow<'static, str> = Cow::Borrowed("--");

    let cargo_config = CargoConfig::default()?;
    let workspace = if let Some(ref manifest) = config.manifest {
        Workspace::new(&Path::new(manifest).canonicalize()?, &cargo_config)?
    } else {
        let root = find_root_manifest_for_wd(config.manifest, cargo_config.cwd())?;
        Workspace::new(&root, &cargo_config)?
    };
    let mut registry = PackageRegistry::new(&cargo_config)?;
    let (_, resolve) = ops::resolve_ws(&workspace)?;

    let mut tw = TabWriter::new(vec!());
    writeln!(&mut tw, "Name\tNow\tCompat\tLatest")?;

    for package in resolve.iter() {
        let (compat_latest, latest) = query_latest(&mut registry, package)?;
        if compat_latest.is_none() && latest.is_none() {
            continue
        }

        writeln!(&mut tw, "{}:\t{}\t{}\t{}",
            package.name(),
            package.version(),
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

    io::stdout().write_all(&tw.into_inner()
        .map_err(|err| io::Error::new(err.error().kind(), err.to_string()))?
    )?;

    Ok(())
}

fn main() {
    let config = Config::from_args();
    start(config).unwrap();
}
