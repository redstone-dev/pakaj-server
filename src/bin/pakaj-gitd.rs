#![allow(dead_code)]
// Daemon to pull packages from their git repos

use std::io::Result;
use std::fs::{read_dir, read_to_string};
use std::path::Path;
use spinners::{Spinner, Spinners};
use std::process::Command;
use git2::{self, Repository};
use core::time::Duration;
use iceoryx2::prelude::*;

struct GitDaemon {}

impl GitDaemon {
    pub fn update_all_packages() -> Result<()> {
        read_dir("packages")
            .unwrap()
            .for_each(|package_directory_path| {
                let path = &package_directory_path.unwrap().path();
                
                // pull from repo if package.git.remote is true, to update package
                // also trigger build script
                let pkg_meta_toml = toml::from_str::<toml::Table>(&read_to_string(&path).unwrap()).unwrap();
                if pkg_meta_toml["package"]["git"]["remote"].as_bool().unwrap() {
                    // Use git cli because `git pull` with git2-rs is a few hundred loc
                    Command::new("git")
                        .arg("pull")
                        .output()
                        .unwrap_or_else(|err| panic!("gitd: {:?}: git pull failed :(\n{}", path, err.to_string()));
                }
            });
        Ok(())
    }
    
    pub fn add_repo(repo_url: String) -> Result<()> {
        let repo = Repository::clone(&repo_url, Path::new("packages"));
        match repo {
            Ok(_) => (),
            Err(err) => eprintln!("gitd: {:?}: could not clone :(\n{}", repo_url, err.to_string())
        }
    
        Ok(())
    }

    pub fn run_cli_listener() -> Result<()> {
        let node = NodeBuilder::new().create::<ipc::Service>().unwrap();

        let event = node.service_builder(&"CliEvent".try_into().unwrap())
            .event()
            .open_or_create().unwrap();

        let listener = event.listener_builder().create().unwrap();

        while node.wait(Duration::ZERO).is_ok() {
            if let Ok(Some(event_id)) = listener.timed_wait_one(Duration::from_secs(1)) {
                println!("event was triggered with id: {:?}", event_id);
            }
        }

        Ok(())
    }
}

fn main() {
    let mut update_sp = Spinner::new(Spinners::Dots12, "Updating all packages...".into());
    GitDaemon::update_all_packages().unwrap();
    update_sp.stop();

    // Start daemon
    match GitDaemon::run_cli_listener() {
        Ok(_) => (),
        Err(err) => eprintln!("gitd: error in GitDaemon\n{}", err.to_string())
    }
}