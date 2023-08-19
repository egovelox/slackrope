pub use crate::cli::Cli;
use log::{debug, info};

pub fn set_logger(cli: &Cli) {
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => {
            let _ = env_logger::builder()
                .filter_level(log::LevelFilter::Warn)
                .try_init();
        }
        1 => {
            let _ = env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .filter_module("skim", log::LevelFilter::Info)
                .filter_module("tuikit", log::LevelFilter::Info)
                .filter_module("html5ever", log::LevelFilter::Info)
                .filter_module("reqwest", log::LevelFilter::Info)
                .filter_module("mio", log::LevelFilter::Info)
                .filter_module("want", log::LevelFilter::Info)
                .try_init();
            info!("Debug mode: info");
        }
        2 => {
            let _ = env_logger::builder()
                .filter_level(log::LevelFilter::max())
                .filter_module("skim", log::LevelFilter::Info)
                .filter_module("tuikit", log::LevelFilter::Info)
                .filter_module("html5ever", log::LevelFilter::Info)
                .filter_module("reqwest", log::LevelFilter::Info)
                .filter_module("mio", log::LevelFilter::Info)
                .filter_module("want", log::LevelFilter::Info)
                .try_init();
            debug!("Debug mode: debug");
        }
        _ => eprintln!("-d flag (debug-mode) was used more than twice."),
    }
}
