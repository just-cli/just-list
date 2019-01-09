use just_core::kernel::{AvailableDownloads, InstalledPackages};
use semver::Version;
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "list")]
struct Opt {
    #[structopt(long = "installed")]
    pub installed: bool,
    #[structopt(long = "cached")]
    pub cached: bool,
    #[structopt(long = "without-versions")]
    pub without_versions: bool,
    #[structopt(long = "with-paths")]
    pub with_paths: bool,
}

impl Opt {
    fn with_versions(&self) -> bool {
        !self.without_versions
    }
}

fn list_installed(opt: &Opt, packages: &InstalledPackages) {
    println!(" > Installed packages:");
    for (pkg_name, pkg_version) in packages.get_packages().iter() {
        print!("\t - {}", pkg_name);
        if opt.with_versions() {
            print!(" ({})", pkg_version);
        }
        println!();
    }
}

fn list_cached(opt: &Opt, downloads: &AvailableDownloads) {
    println!(" > Cached packages:");
    for (pkg_name, pkg_variants) in downloads.get_downloads().iter() {
        print!("\t - {}", pkg_name);
        if opt.with_paths || opt.with_versions() {
            list_cached_variants(opt, pkg_variants);
        }
        println!();
    }
}

fn list_cached_variants(opt: &Opt, pkg_variants: &HashMap<Version, PathBuf>) {
    for (pkg_version, path) in pkg_variants.iter() {
        if opt.with_paths && opt.with_versions() {
            print!(" ({} in {:?})", pkg_version, path);
        } else if opt.with_versions() {
            print!(" ({})", pkg_version);
        } else {
            print!(" (in {:?})", path);
        }
        println!();
    }
}

fn list(opt: &Opt) {
    use just_core::kernel::Kernel;

    let kernel = Kernel::load();

    if opt.installed {
        list_installed(opt, &kernel.packages);
    }

    if opt.cached {
        list_cached(opt, &kernel.downloads);
    }
}

fn main() {
    let mut opt: Opt = Opt::from_args();
    if !opt.installed {
        opt.installed = !opt.cached;
    }

    list(&opt);
}
