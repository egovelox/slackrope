use clap::Parser;
use log::{debug, info};
use std::process::exit;
use sysinfo::{System, SystemExt};

mod cli;
mod logger;
mod models;
mod utils;
mod weechat_connection;
mod weechat_hotlist;
mod weechat_process;
mod weechat_slack;

use weechat_hotlist::{hotlist, clear_hotlist, HotlistFlags};
use weechat_process::{kill_weechat_processes, print_register_url, WeechatSpawnFailed};
use weechat_slack::{list_registered_slack_teams, register_slack_token};

fn main() {
    let mut system = System::new_all();
    let cli = cli::Cli::parse();
    logger::set_logger(&cli);

    match cli.command {
        cli::Commands::Kill => fold(kill_weechat_processes(&mut system)),
        cli::Commands::ListTeams => fold(list_registered_slack_teams(&mut system)),
        cli::Commands::Hotlist { format } => fold(hotlist(&system, HotlistFlags { format })),
        cli::Commands::Clear => fold(clear_hotlist(&system)),
        cli::Commands::Register { token } => match token {
            Some(token) => fold(register_slack_token(&mut system, &token)),
            None => fold(print_register_url()),
        },
    };

    info!("Exiting !");
    exit(0);
}

fn fold<O>(result: Result<O, anyhow::Error>) {
    if let Err(error) = result {
        if let Some(error) = error.downcast_ref::<WeechatSpawnFailed>() {
            debug!("{}", error);
            info!(
                "Please check that {} is present in $PATH",
                error.program_name
            );
        } else {
            debug!("{}", error);
        }

        exit(1)
    }
}
