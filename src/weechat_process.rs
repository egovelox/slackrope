use anyhow::{Context, Result};
use log::debug;
use std::error;
use std::fmt;
use sysinfo::{Process, ProcessExt, System, SystemExt};

fn get_weechat_processes(sys: &System) -> Option<Vec<&Process>> {
    let mut vec: Vec<&Process> = Vec::new();
    for process in sys.processes_by_name("weechat") {
        let name = process.name();
        if name == "weechat" || name == "weechat-headless" {
            vec.push(process);
            debug!(
                "Found running {} process with pid: {}",
                process.name(),
                process.pid()
            );
        }
    }
    if vec.len() == 0 {
        debug!("Did not found any running weechat processes");
        return None;
    }
    Some(vec)
}

pub fn kill_weechat_processes(sys: &mut System) -> Result<()> {
    debug!("...kill_weechat_processes ?");
    if let Some(processes) = get_weechat_processes(sys) {
        for process in processes {
            process.kill();
        }
        debug!("Done killing weechat processes");
    }
    Ok(())
}

pub fn is_weechat_running(sys: &System) -> bool {
    debug!("...is_weechat_running ?");
    get_weechat_processes(sys).is_some()
}

pub fn spawn_weechat_process() -> Result<std::process::Child> {
    let program_name = "weechat-headless";
    debug!("...spawning {} process", program_name);
    std::process::Command::new(program_name)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .context(WeechatSpawnFailed {
            program_name: program_name.to_string(),
        })
}

#[derive(Debug)]
pub struct WeechatSpawnFailed {
    pub program_name: String,
}

impl fmt::Display for WeechatSpawnFailed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Error WeechatSpawnFailed] program_name: {}",
            self.program_name
        )
    }
}

impl error::Error for WeechatSpawnFailed {
    fn description(&self) -> &str {
        "io-error"
    }
}

pub fn print_register_url() -> Result<()> {
    let register_baseurl = "https://slack.com/oauth/authorize";
    let client_id = "2468770254.51917335286";
    let scope = "client";
    let redirect_uri = "https%3A%2F%2Fwee-slack.github.io%2Fwee-slack%2Foauth";

    println!("To register a new slack workspace, please follow this link:");
    println!();
    println!(
        "   {}?client_id={}&scope={}&redirect_uri={}",
        register_baseurl, client_id, scope, redirect_uri
    );
    println!();
    println!("You should then be able to get a token, and use it to run the command: ");
    println!();
    println!("  register --token <TOKEN>");
    Ok(())
}
