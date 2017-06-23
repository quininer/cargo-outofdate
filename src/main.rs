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
use cargo::core::SourceId;
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
struct Options {
    /// manifest path
    #[structopt(short = "m", long = "manifest")]
    manifest: Option<String>,

    /// only check root dependencies
    #[structopt(short = "R", long = "only-root")]
    only_root: bool,

    /// update crates-io
    #[structopt(short = "U", long = "update-crates-io")]
    update_crates_io: bool,

    /// TODO https://github.com/TeXitoi/structopt/issues/1
    #[structopt(hidden = true)]
    #[doc(hidden)]
    _ignore: Option<String>
}


#[inline]
fn start(options: Options) -> CargoResult<()> {
    let config = CargoConfig::default()?;
    let workspace = if let Some(ref manifest) = options.manifest {
        Workspace::new(&Path::new(manifest).canonicalize()?, &config)?
    } else {
        let root = find_root_manifest_for_wd(options.manifest, config.cwd())?;
        Workspace::new(&root, &config)?
    };
    let mut registry = PackageRegistry::new(&config)?;
    let (_, resolve) = ops::resolve_ws(&workspace)?;
    let package = workspace.current()?;

    if options.update_crates_io {
        SourceId::crates_io(&config)?
            .load(&config)
            .update()?;
    }

    let mut results = Vec::new();

    for pkg in resolve.iter() {
        if options.only_root && !package.dependencies()
            .iter()
            .any(|dep| dep.matches_id(pkg))
        {
            continue
        }

        let (compat_latest, latest) = query_latest(&mut registry, pkg)?;
        if compat_latest.is_none() && latest.is_none() {
            continue
        }

        results.push((pkg, compat_latest, latest));
    }

    if results.is_empty() {
        config.shell()
            .say("All dependencies are up to date, yay!", 0)?;
    } else {
        results.sort_by_key(|&(pkg, _, _)| pkg.name());

        let mut tw = TabWriter::new(vec!());
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

        io::stdout().write_all(&tw.into_inner()
            .map_err(|err| io::Error::new(err.error().kind(), err.to_string()))?
        )?;
    }

    Ok(())
}

fn main() {
    let options = Options::from_args();
    start(options).unwrap();
}
