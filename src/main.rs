use std::collections::HashSet;
use std::fs::File;
use std::process::exit;

use clap::Parser;
use dotenv::dotenv;
use log::{error, info};

use serde_xml_rs::from_reader;

mod types;
use types::args::CommandArg;
use types::package::{JSPackage, MetaPackage, Mode};
use url::Url;

mod api;

fn get_package_common<T>(
    mode: &Mode,
    action: impl FnOnce() -> Result<JSPackage, Box<dyn std::error::Error>>,
    error_context: T,
) -> Option<JSPackage>
where
    T: std::fmt::Display,
{
    match action() {
        Ok(pkg) => Some(pkg),
        Err(err) => {
            let msg = match mode {
                Mode::Real => format!("failed request: {}: {}", error_context, err),
                Mode::Test => format!("failed read file {}: {}", error_context, err),
            };
            error!("{}", msg);
            None
        }
    }
}

fn get_main_package_depends(package: &MetaPackage, mode: &Mode) -> Option<JSPackage> {
    get_package_common(
        mode,
        || match mode {
            Mode::Real => package.request_main_get_package(),
            Mode::Test => package.file_get_package(),
        },
        "main package",
    )
}

fn get_package(mode: &Mode, depend_package: &JSPackage) -> Option<JSPackage> {
    get_package_common(
        mode,
        || match mode {
            Mode::Real => depend_package.request_get_package(),
            Mode::Test => depend_package.file_get_package(),
        },
        format!("{}/package.json", depend_package.name),
    )
}

fn build_dependencies_tree(
    package: JSPackage,
    mode: &Mode,
    max_depth: i32,
    current_depth: i32,
    visited: &mut HashSet<String>,
) -> JSPackage {
    if current_depth >= max_depth {
        return package;
    }

    if !visited.insert(package.name.clone()) {
        // TODO: fix this in future, check package version
        error!("cycle detected at {}", package.name);
        return package;
    }

    let mut result = package.clone();
    let mut new_deps = Vec::new();

    for dep in &package.dependencies {
        if let Some(dep_pkg) = get_package(mode, dep) {
            let full_dep_tree =
                build_dependencies_tree(dep_pkg, mode, max_depth, current_depth + 1, visited);
            new_deps.push(full_dep_tree);
        }
    }

    result.dependencies = new_deps;
    visited.remove(&package.name);

    result
}

fn load_package(package_path: &String) {
    let file = File::open(package_path).unwrap_or_else(|err| {
        error!("failed open package file: {}", err);
        exit(1);
    });

    let package: MetaPackage = from_reader(file).unwrap_or_else(|err| {
        error!("failed parse {}: {}", package_path, err);
        exit(1);
    });

    match package.get_mode() {
        Ok(mode) => {
            if mode == Mode::Real && Url::parse(&package.url).is_err() {
                error!("'{}' is invalid url", package.url);
                exit(1);
            }

            if let Some(main_package) = get_main_package_depends(&package, &mode) {
                let mut visited = HashSet::new();
                let tree =
                    build_dependencies_tree(main_package, &mode, package.depth, 0, &mut visited);

                ptree::print_tree(&tree).unwrap();
            }
        }
        Err(error) => {
            error!("{}", error);
            exit(1);
        }
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let shell_args = CommandArg::parse();
    load_package(&shell_args.package);
}
