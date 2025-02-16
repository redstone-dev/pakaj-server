#![allow(dead_code)]
// Daemon to pull packages from their git repos

use std::fmt::Debug;
use std::io::Result;
use std::fs::{read_dir, read_to_string};
use spinners::{Spinner, Spinners};
use std::process::Command;

fn update_all_packages() -> Result<()> {
    read_dir("packages")
        .unwrap()
        .for_each(|package_directory_path| {
            let path = package_directory_path.unwrap().path();
            
            // pull from repo if package.git.remote is true, to update package
            // also trigger build script
            let pkg_meta_toml = toml::from_str::<toml::Table>(&read_to_string(path).unwrap()).unwrap();
            if pkg_meta_toml["package"]["git"]["remote"].as_bool().unwrap() {
                Command::new("git")
                    .args(["pull"])
                    .output()
                    .unwrap_or_else(|_| panic!("{:?}: git pull failed :(", &path));
            }
        });
    Ok(())
}

fn main() {
    let mut update_sp = Spinner::new(Spinners::Dots12, "Updating all packages...".into());
    update_all_packages().unwrap();
    update_sp.stop();
}