use std::fs::File;

use clap::Parser;
use dotenv::dotenv;
use log::{error, info};

use serde_xml_rs::{from_reader, Error};

mod types;
use types::args::CommandArg;
use types::package::Package;

fn load_package(package_path: &String) {
    let try_file = File::open(package_path);

    match try_file {
        Ok(file) => {
            let try_package: Result<Package, Error> = from_reader(file);
            match try_package {
                Ok(package) => {
                    info!("Success parse {}: ", package_path);
                    info!("name: {}", package.name);
                    info!("url: {}", package.url);
                    info!("mode: {}", package.mode);
                    info!("output: {}", package.output);
                    info!("depth: {}", package.depth);
                }
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
