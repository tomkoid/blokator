#![allow(unreachable_code)]

use clap::Parser;
use dirs::home_dir;

use std::process::exit;

pub mod actions;
mod allowed_exit_functions;
mod android;
mod arguments;
pub mod colors;
mod commands;
mod copy;
pub mod error;
mod handle_permissions;
mod initialize_dirs;
pub mod logging;
pub mod messages;
pub mod presets;
pub mod read;
mod repos;
mod services;
mod sync;
pub mod tor;
pub mod write;

#[cfg(target_family = "windows")]
mod windows;

use crate::commands::exec_command;
use crate::logging::Logger;
#[cfg(target_family = "windows")]
use crate::windows::is_elevated;
use arguments::Args;

use crate::allowed_exit_functions::check_allowed_function;
use crate::colors::Colors;
use crate::handle_permissions::handle_permissions;
use crate::initialize_dirs::{already_initialized, initialize_dir};
use crate::messages::Messages;
use crate::read::read_file_to_string;

#[cfg(target_family = "unix")]
const HOSTS_FILE: &str = "/etc/hosts";
#[cfg(target_family = "unix")]
const HOSTS_FILE_BACKUP_PATH: &str = "/etc/hosts.backup";

#[cfg(target_family = "windows")]
const HOSTS_FILE: &str = r"C:\Windows\System32\drivers\etc\hosts";
#[cfg(target_family = "windows")]
const HOSTS_FILE_BACKUP_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts.backup";

pub const SPINNER_TYPE: spinners::Spinners = spinners::Spinners::Dots2;

#[derive(PartialEq, Eq)]
pub enum Actions {
    Restore,
    Backup,
    Apply,
}

#[derive(Clone)]
pub struct AppState {
    pub args: Args,
    pub logger: Logger,
    pub colors: Colors,
    pub messages: Messages,
}

// global Logger
thread_local! {
    static LOGGER: Logger = Logger::new(&Messages::new());
}

#[tokio::main]
async fn main() {
    // Initialize colors
    let colors: Colors = Colors::new();

    // Initialize messages
    let messages: Messages = Messages::new();

    // Initialize logger
    let logger = Logger::new(&messages);

    // Parse arguments
    let args = Args::parse();

    // Initialize state
    let state = AppState {
        args: args.clone(),
        logger: logger.clone(),
        colors: colors.clone(),
        messages: messages.clone(),
    };

    // Check if user is running blokator as root / administrator, otherwise exit
    handle_permissions(&state);

    // Initialize important directories if they are not already initialized
    if !already_initialized() {
        initialize_dir();
    }

    exec_command(&state).await;

    // Check if allowed exit functions ended (else exit)
    check_allowed_function(&args);

    logger.log_error("no_action_specified");
    logger.log_help("no_action_specified");

    // println!(
    //     "{}error:{} {}",
    //     colors.bold_red,
    //     colors.reset,
    //     messages.message.get("no_action_specified").unwrap()
    // );
    //
    // println!(
    //     "{}HELP:{} {}",
    //     colors.bold_green,
    //     colors.reset,
    //     messages.help_message.get("no_action_specified").unwrap()
    // );
    exit(1);
}

pub fn get_data_dir() -> String {
    format!("{}/.local/share/adblocker", home_dir().unwrap().display())
}
