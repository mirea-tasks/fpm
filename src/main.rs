use std::fs::File;
use std::process::exit;

use clap::Parser;
use dotenv::dotenv;
use log::{error, info};

use serde_xml_rs::{from_reader, Error};

mod types;
use types::args::CommandArg;
use types::package::{Mode, Package};

mod api;

fn get_package_depends(package: &Package, mode: &Mode) {
    let result = match mode {
        Mode::Real => package.request_main_get_package(),
        Mode::Test => package.file_get_package(),
    };

    match result {
        Ok(req_package) => {
            info!(
                "package name: {} ({})",
                req_package.name, req_package.version
            );
            if let Some(deps) = req_package.dependencies {
                for (name, version) in deps {
                    info!("{name} -> {version}");
                }
            } else {
                println!("not depends");
            }
        }
        Err(error) => match mode {
            Mode::Real => error!("failed request: {}", error),
            Mode::Test => error!("failed read file: {}", error),
        },
    }
}

fn load_package(package_path: &String) {
    let try_file = File::open(package_path);

    match try_file {
        Ok(file) => {
            let try_package: Result<Package, Error> = from_reader(file);
            match try_package {
                Ok(package) => match package.get_mode() {
                    Ok(mode) => {
                        get_package_depends(&package, &mode);
                    }
                    Err(error) => {
                        error!("{}", error);
                        exit(1);
                    }
                },
                Err(error) => {
                    error!("failed parse {}: {}", package_path, error);
                }
            }
        }
        Err(error) => {
            error!("failed open package file: {}", error);
        }
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let shell_args = CommandArg::parse();
    load_package(&shell_args.package);
}
