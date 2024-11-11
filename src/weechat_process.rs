use anyhow::{Context, Result};
use log::debug;
use std::error;
use std::fmt;
use sysinfo::{Process, ProcessExt, System, SystemExt};

use crate::environment::{get_config, WEECHAT_PROGRAM_NAME};

pub fn get_weechat_processes(sys: &System) -> Option<Vec<&Process>> {
    let program_name = get_config::<String>(WEECHAT_PROGRAM_NAME);
    let mut vec: Vec<&Process> = Vec::new();
    for process in sys.processes_by_name("weechat") {
        let name = process.name();
        if name == "weechat" || name == program_name {
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
    sys.refresh_processes();
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
    let program_name = get_config::<String>(WEECHAT_PROGRAM_NAME);

    debug!("...spawning {} process", &program_name);
    std::process::Command::new(&program_name)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .context(WeechatSpawnFailed { program_name })
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
