use crate::initialize_colors::initialize_colors;
use crate::services::init::Init;
use crate::services::init::NetworkManager;
use crate::Messages;

pub fn restart_networkmanager() {
    let colors = initialize_colors();
    let messages: Messages = Messages::new();

    if NetworkManager::exists() {
        print!(
            "{}  >{} Restarting NetworkManager..",
            colors.bold_blue, colors.reset
        );

        let networkmanager_status = match NetworkManager::restart() {
            Ok(s) => s,
            Err(e) => panic!("couldn't restart NetworkManager: {e}"),
        };

        if networkmanager_status.success() {
            println!(" {}done{}", colors.bold_green, colors.reset);
        } else {
            // Init 2 = OpenRC
            /*
             * OpenRC sometime returns 1 as a exit code when printing errors and
             * warning, which is the same exit code
             */
            if Init::get_init() == Some(Init::OpenRC) {
                println!(" {}failed / warning{}", colors.bold_red, colors.reset);
            } else {
                println!(" {}failed{}", colors.bold_red, colors.reset);
            }
        }
    } else {
        println!(
            "{}==>{} {}",
            colors.bold_yellow,
            colors.reset,
            messages
                .message
                .get("networkmanager_restart_manually")
                .unwrap()
        );
    }
}
