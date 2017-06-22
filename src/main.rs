#![feature(attr_literals)]

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


const EMPTY_VERSION: Cow<'static, str> = Cow::Borrowed("--");

#[derive(StructOpt)]
#[structopt]
struct Config {
    /// manifest path
    #[structopt(short = "m", long = "manifest")]
    manifest: Option<String>,

    /// only check root dependencies
    #[structopt(short = "R", long = "only-root")]
    only_root: bool,

    /// TODO https://github.com/TeXitoi/structopt/issues/1
    #[structopt(hidden = true)]
    #[doc(hidden)]
    _ignore: Option<String>
}


#[inline]
fn start(config: Config) -> CargoResult<()> {
    let cargo_config = CargoConfig::default()?;
    let workspace = if let Some(ref manifest) = config.manifest {
        Workspace::new(&Path::new(manifest).canonicalize()?, &cargo_config)?
    } else {
        let root = find_root_manifest_for_wd(config.manifest, cargo_config.cwd())?;
        Workspace::new(&root, &cargo_config)?
    };
    let mut registry = PackageRegistry::new(&cargo_config)?;
    let (_, resolve) = ops::resolve_ws(&workspace)?;
    let package = workspace.current()?;

    let mut yay = false;
    let mut tw = TabWriter::new(vec!());
    writeln!(&mut tw, "Name\tNow\tCompat\tLatest")?;

    for pkg in resolve.iter() {
        if config.only_root {
            if !package.dependencies()
                .iter()
                .any(|dep| dep.matches_id(pkg))
            {
                continue
            }
        }

        let (compat_latest, latest) = query_latest(&mut registry, pkg)?;
        if compat_latest.is_none() && latest.is_none() {
            continue
        }

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

        yay = true;
    }

    if yay {
        io::stdout().write_all(&tw.into_inner()
            .map_err(|err| io::Error::new(err.error().kind(), err.to_string()))?
        )?;
    } else {
        cargo_config
            .shell()
            .say("All dependencies are up to date, yay!", 0)?;
    }

    Ok(())
}

fn main() {
    let config = Config::from_args();
    start(config).unwrap();
}
