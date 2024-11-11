use std::env;
use std::path::Path;

use anyhow::Result;
use sysinfo::{ProcessExt, System};

use crate::environment::CONFIG_FILE;
pub use crate::environment::{
    get_config, APP_NAME, SLACK_REGISTER_BASEURL, SLACK_REGISTER_REDIRECT_URI,
    SLACK_REGISTER_SCOPE, SLACK_REGISTER_WEESLACK_CLIENT_ID, WEECHAT_HOST, WEECHAT_PASSWORD,
    WEECHAT_PROGRAM_NAME, WEECHAT_RELAY_PORT, WEE_SLACK_PLUGIN_DIRECTORY,
    WEE_SLACK_PLUGIN_FILENAME,
};
use crate::utils::sleep;
use crate::weechat_connection::init_connection;
use crate::weechat_process::{
    get_weechat_processes, is_weechat_running, kill_weechat_processes, spawn_weechat_process,
};
use crate::weechat_slack::check_connection_and_python_wee_slack_plugin;

pub fn print_weechat_health(sys: &mut System) -> Result<()> {
    let report_lines = build_weechat_health_report(sys)?;
    for line in report_lines {
        println!("{line}")
    }
    Ok(())
}

fn build_weechat_health_report(sys: &mut System) -> Result<Vec<String>> {
    let mut report_lines = Vec::<String>::new();
    let mut nl = |line| report_lines.push(line);

    let is_weechat_running = is_weechat_running(sys);
    /* WEECHAT INFO */
    nl(format!("> weechat INFO"));
    nl(format!("is_running: {}", is_weechat_running));

    if let Some(processes) = get_weechat_processes(sys) {
        for (i, process) in processes.iter().enumerate() {
            nl(format!("process#{i}_name: {}", process.name()));
            nl(format!("process#{i}_exec: {}", process.exe().display()));
            nl(format!("process#{i}_pid: {}", process.pid()));
        }
    }

    if !is_weechat_running {
        spawn_weechat_process()?;
        sleep(2);
    }
    let (weechat_connection_state, wee_slack_plugin_state) = test_connection_and_plugin();
    nl(format!(
        "weechat_connection_test: {}",
        weechat_connection_state
    ));
    nl(format!("weeslack_plugin_test: {}", wee_slack_plugin_state));
    nl(format!(
        "weeslack_plugin_install_path: {}",
        get_wee_slack_plugin_install_path()
    ));
    nl(format!(""));

    /* APP INFO */
    nl(format!("> {APP_NAME} CONFIG"));
    nl(format!("current_exe: {}", get_current_exec_path()));
    nl(format!("current_config: {}", get_current_config_path()));
    print_app_loaded_config(nl);

    /* if weechat WAS not running,
     * we prefer to kill the instances we used
     * for our health tests, etc.
     */
    if !is_weechat_running {
        kill_weechat_processes(sys)?
    }

    Ok(report_lines)
}

fn print_app_loaded_config(mut nl: impl FnMut(String)) {
    nl(format!(
        "{WEECHAT_HOST}: {}",
        get_config::<String>(WEECHAT_HOST)
    ));
    nl(format!(
        "{WEECHAT_RELAY_PORT}: {}",
        get_config::<String>(WEECHAT_RELAY_PORT)
    ));
    nl(format!(
        "{WEECHAT_PROGRAM_NAME}: {}",
        get_config::<String>(WEECHAT_PROGRAM_NAME)
    ));
    nl(format!(
        "{WEECHAT_PASSWORD}: {}",
        get_config::<String>(WEECHAT_PASSWORD)
            .chars()
            .map(|_| '*')
            .collect::<String>()
    ));
    nl(format!(
        "{SLACK_REGISTER_BASEURL}: {}",
        get_config::<String>(SLACK_REGISTER_BASEURL)
    ));
    nl(format!(
        "{SLACK_REGISTER_WEESLACK_CLIENT_ID}: {}",
        get_config::<String>(SLACK_REGISTER_WEESLACK_CLIENT_ID)
    ));
    nl(format!(
        "{SLACK_REGISTER_SCOPE}: {}",
        get_config::<String>(SLACK_REGISTER_SCOPE)
    ));
    nl(format!(
        "{SLACK_REGISTER_REDIRECT_URI}: {}",
        get_config::<String>(SLACK_REGISTER_REDIRECT_URI)
    ));
    nl(format!(
        "{WEE_SLACK_PLUGIN_DIRECTORY}: {}",
        get_config::<String>(WEE_SLACK_PLUGIN_DIRECTORY)
    ));
    nl(format!(
        "{WEE_SLACK_PLUGIN_FILENAME}: {}",
        get_config::<String>(WEE_SLACK_PLUGIN_FILENAME)
    ));
}

fn get_wee_slack_plugin_install_path() -> String {
    let wee_slack_plugin_directory = get_config::<String>(WEE_SLACK_PLUGIN_DIRECTORY);
    let wee_slack_plugin_filename = get_config::<String>(WEE_SLACK_PLUGIN_FILENAME);
    let wee_slack_plugin_filepath =
        format!("{wee_slack_plugin_directory}/{wee_slack_plugin_filename}",);
    let wee_slack_plugin_install_path = match Path::new(&wee_slack_plugin_filepath.clone())
        .try_exists()
    {
        Ok(exists) => {
            if exists {
                wee_slack_plugin_filepath
            } else {
                format!(
                    "File {wee_slack_plugin_filename} not found in directory {wee_slack_plugin_directory}",
                )
            }
        }
        Err(_) => {
            format!("Failure while checking if file {wee_slack_plugin_filepath} exists",)
        }
    };
    wee_slack_plugin_install_path
}

fn test_connection_and_plugin() -> (String, String) {
    let (weechat_connection_state, wee_slack_plugin_state) = match init_connection() {
        Ok(mut connection) => match check_connection_and_python_wee_slack_plugin(&mut connection) {
            Ok((is_connected, is_plugin_loaded)) => (
                if is_connected { "ok" } else { "ko" },
                if is_plugin_loaded { "ok" } else { "ko" },
            ),
            Err(_) => ("ko", "ko"),
        },
        Err(_) => ("ko", "ko"),
    };
    (
        weechat_connection_state.to_string(),
        wee_slack_plugin_state.to_string(),
    )
}

fn get_current_exec_path() -> String {
    match env::current_exe() {
        Ok(exe_path) => exe_path.display().to_string(),
        Err(_) => "unknown".to_string(),
    }
}

fn get_current_config_path() -> String {
    match get_config::<Option<String>>(CONFIG_FILE) {
        Some(config_path) => config_path,
        None => "none".to_string(),
    }
}
