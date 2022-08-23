#![feature(io_error_uncategorized)]

use clap::Parser;
use dirs::home_dir;
use restore::restore;
use std::process::exit;
use nix::unistd::Uid;
use std::path::Path;

pub mod read;
pub mod write;
mod backup;
mod restore;
mod sync;
mod apply;
mod systemd;

use crate::systemd::networkmanager::{ 
                                    networkmanager_exists,
                                    networkmanager_restart
                                    };
use crate::backup::backup;
use crate::sync::sync;
use crate::apply::apply;

const HOSTS_FILE: &str = "/etc/hosts";
const HOSTS_FILE_BACKUP_PATH: &str = "/etc/hosts.backup";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = "Easy system-wide adblocker")]
struct Args {
    /// Start the adblocker
    #[clap(short, long, value_parser, default_value_t = true)]
    start: bool,

    /// Sync the adblocker
    #[clap(short = 'S', long, value_parser, default_value_t = false)]
    sync: bool,

    /// Restore the /etc/hosts backup
    #[clap(short = 'r', long, value_parser, default_value_t = false)]
    restore: bool
}

fn main() {
    let args = Args::parse();

    // Check if the program is running with root permissions
    if !Uid::effective().is_root() {
        println!("==> Root is required to run the adblocker.");
        exit(1);
    }
    
    if args.sync {
        sync();
        println!("==> Synced the adblocker.");
        exit(0);
    }

    // Restore backup from /etc/hosts.backup to /etc/hosts
    if args.restore {
        restore(HOSTS_FILE_BACKUP_PATH, HOSTS_FILE);
        if networkmanager_exists() {
            let networkmanager_status = match networkmanager_restart() {
                Ok(s) => s,
                Err(e) => panic!("{}", e)
            };

            if networkmanager_status.success() {
                println!("==> Restarted NetworkManager.service.");
            } else {
                println!("==> Cannot restart NetworkManager.service.")
            }
        } else {
            println!("==> To apply the changes, manually restart your networking service or restart the system");
        }
        println!("==> Restored the backup.");
        exit(0);
    }

    if args.start {
        let local_hosts = format!(
            "{}/hosts",
            get_data_dir()
        );
        
        if !Path::new(HOSTS_FILE_BACKUP_PATH).exists() {
            // Backup /etc/hosts to /etc/hosts.backup
            backup(HOSTS_FILE, HOSTS_FILE_BACKUP_PATH); 
        }

        // Rewrite /etc/hosts
        apply(&local_hosts, HOSTS_FILE);
        
        if networkmanager_exists() {
            let networkmanager_status = match networkmanager_restart() {
                Ok(s) => s,
                Err(e) => panic!("{}", e)
            };

            if networkmanager_status.success() {
                println!("==> Restarted NetworkManager.service.");
            } else {
                println!("==> Cannot restart NetworkManager.service.")
            }
        } else {
            println!("==> To apply the changes, manually restart your networking service or restart the system");
        }

        println!("==> Started the adblocker.");
        exit(0);
    }
}

fn get_data_dir() -> String {
    format!(
        "{}/.local/share/adblocker",
        home_dir().unwrap().display()
    )
}