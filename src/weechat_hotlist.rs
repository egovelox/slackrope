pub use crate::cli::OutputFormat;
pub use crate::models::{Buffer, Detailed, DetailedHotlist, SimpleHotlist};
pub use crate::utils::{clean_string, match_string, to_utf8_lossy};
pub use crate::weechat_connection::init_connection;
pub use crate::weechat_process::{is_weechat_running, spawn_weechat_process};
use anyhow::Result;
use log::{debug, info};
use sysinfo::System;
use weechat_relay_rs::commands::{Command, InfolistCommand, StrArgument, InputCommand, PointerOrName};
use weechat_relay_rs::messages::{InfolistItem, Object, WInfolist};
use weechat_relay_rs::Connection;

pub struct HotlistFlags {
    pub format: OutputFormat,
}

pub fn hotlist(sys: &System, flags: HotlistFlags) -> Result<()> {
    match flags.format {
        OutputFormat::Shell => { 
            // As this format is to be used by other shell programs,
            // we don't spawn weechat if it's not currently running
            if !is_weechat_running(sys) {
                println!("-");
                return Ok(())
            }
            print_shell_hotlist()? 
        },
        OutputFormat::Simple => { 
            if !is_weechat_running(sys) {
                spawn_weechat_process()?;
            }
            print_simple_hotlist()? 
        },
        OutputFormat::Detailed => { 
            if !is_weechat_running(sys) {
                spawn_weechat_process()?;
            }
            print_detailed_hotlist()? 
        },
    };
    Ok(())
}

pub fn clear_hotlist(sys: &System) -> Result<()> {
    if !is_weechat_running(sys) {
        debug!("Did not clear hotlist : weechat is currently not running");
        return Ok(())
    }
    let mut connection = init_connection("127.0.0.1:8000", "password")?;
    debug!("connection initiated");
    send_clear_hotlist_request(&mut connection)?;
    debug!("clear hotlist request sent");
    Ok(())
}

fn print_shell_hotlist() -> Result<()> {
    let mut connection = init_connection("127.0.0.1:8000", "password")?;
    debug!("connection initiated");
    send_hotlist_request(&mut connection)?;
    debug!("hotlist request sent");
    let hotlist = get_hotlist_response(&mut connection)?;
    debug!("hotlist response received");

    let simple_hotlist = build_simple_hotlist(&hotlist)?;
    println!("{} {} {}", 
        simple_hotlist.priority_1,
        simple_hotlist.priority_2,
        simple_hotlist.priority_3,
    );
    Ok(())
}

fn print_simple_hotlist() -> Result<()> {
    let mut connection = init_connection("127.0.0.1:8000", "password")?;
    debug!("connection initiated");
    send_hotlist_request(&mut connection)?;
    debug!("hotlist request sent");
    let hotlist = get_hotlist_response(&mut connection)?;
    debug!("hotlist response received");

    let simple_hotlist = build_simple_hotlist(&hotlist)?;
    let serialized = serde_json::to_string(&simple_hotlist).unwrap();
    println!("{}", serialized);
    Ok(())
}

fn print_detailed_hotlist() -> Result<()> {
    let mut connection = init_connection("127.0.0.1:8000", "password")?;
    debug!("connection initiated");
    send_hotlist_request(&mut connection)?;
    debug!("hotlist request sent");
    let hotlist = get_hotlist_response(&mut connection)?;
    debug!("hotlist response received");

    let detailed_hotlist = build_detailed_hotlist(&hotlist)?;
    let serialized = serde_json::to_string(&detailed_hotlist).unwrap();
    println!("{}", serialized);
    Ok(())
}

fn send_hotlist_request(connection: &mut Connection) -> Result<()> {
    let info_command = InfolistCommand::new(
        StrArgument::new("hotlist").unwrap().to_stringargument(),
        None,
        vec![],
    );
    let info_command: Command<InfolistCommand> = Command {
        id: None,
        command: info_command,
    };
    connection.send_command(&info_command)?;
    Ok(())
}

fn send_clear_hotlist_request(connection: &mut Connection) -> Result<()> {
    let input_command = InputCommand::new(
        PointerOrName::Name(StrArgument::new("core.weechat").unwrap().to_stringargument()),
        StrArgument::new("/hotlist clear").unwrap().to_stringargument(),
    );
    let input_command = Command {
        id: None,
        command: input_command,
    };
    connection.send_command(&input_command)?;
    Ok(())
}

fn get_hotlist_response(connection: &mut Connection) -> Result<Option<WInfolist>> {
    let message = connection.get_message();
    match message {
        Ok(m) => {
            if m.objects.len() > 0 {
                match &m.objects[0] {
                    Object::Inl(infolist) => Ok(Some(infolist.clone())),
                    _ => {
                        info!("Could not parse the hotlist response");
                        Ok(None)
                    }
                }
            } else {
                Ok(None)
            }
        }
        Err(e) => {
            debug!("{:?}", e);
            info!("Could not receive the hotlist response");
            Ok(None)
        }
    }
}

fn build_simple_hotlist(hotlist: &Option<WInfolist>) -> Result<SimpleHotlist> {
    match hotlist {
        Some(hotlist) => {
            let mut priority_1 = 0;
            let mut priority_2 = 0;
            let mut priority_3 = 0;
            for item in hotlist.items.iter() {
                for variable in item.variables.iter() {
                    if match_string(&variable.name, "priority") {
                        if let Some(v) = match variable.value {
                            Object::Int(v) => Some(v),
                            _ => None,
                        } {
                            match v {
                                3 => count_simple(&item, &mut priority_3, 3),
                                2 => count_simple(&item, &mut priority_2, 2),
                                1 => count_simple(&item, &mut priority_1, 1),
                                _ => {}
                            }
                        }
                    }
                }
            }
            Ok(SimpleHotlist {
                priority_1,
                priority_2,
                priority_3,
            })
        }
        None => Ok(SimpleHotlist {
            priority_1: -1,
            priority_2: -1,
            priority_3: -1,
        }),
    }
}

fn build_detailed_hotlist(hotlist: &Option<WInfolist>) -> Result<DetailedHotlist> {
    match hotlist {
        Some(hotlist) => {
            let mut priority_1 = 0;
            let mut priority_2 = 0;
            let mut priority_3 = 0;
            let mut buffers_1: Vec<Buffer> = vec![];
            let mut buffers_2: Vec<Buffer> = vec![];
            let mut buffers_3: Vec<Buffer> = vec![];
            for item in hotlist.items.iter() {
                for variable in item.variables.iter() {
                    if match_string(&variable.name, "priority") {
                        if let Some(v) = match variable.value {
                            Object::Int(v) => Some(v),
                            _ => None,
                        } {
                            match v {
                                3 => count_detailed(&item, &mut priority_3, &mut buffers_3, 3),
                                2 => count_detailed(&item, &mut priority_2, &mut buffers_2, 2),
                                1 => count_detailed(&item, &mut priority_1, &mut buffers_1, 1),
                                _ => {}
                            }
                        }
                    }
                }
            }
            Ok(DetailedHotlist {
                priority_1: Detailed {
                    count: priority_1,
                    items: buffers_1,
                },
                priority_2: Detailed {
                    count: priority_2,
                    items: buffers_2,
                },
                priority_3: Detailed {
                    count: priority_3,
                    items: buffers_3,
                },
            })
        }
        None => Ok(DetailedHotlist {
            priority_1: Detailed {
                count: -1,
                items: Vec::new(),
            },
            priority_2: Detailed {
                count: -1,
                items: Vec::new(),
            },
            priority_3: Detailed {
                count: -1,
                items: Vec::new(),
            },
        }),
    }
}

fn count_simple(item: &InfolistItem, count: &mut i32, priority: u8) {
    if priority != 1 {
        *count += 1
    } else {
        for variable in item.variables.iter() {
            if match_string(&variable.name, "buffer_name") {
                if let Some(buffer_name) = match &variable.value {
                    Object::Str(buffer_name) => Some(clean_string(&buffer_name)),
                    _ => None,
                } {
                    // Filter only slack-thread buffers
                    // A slack-thread buffer name has 4 parts separated by
                    // "."
                    // e.g "slack.workspace.#channel.03f"
                    let split = &buffer_name.split(".").collect::<Vec<&str>>();
                    if split.len() > 3 {
                        *count += 1
                    }
                }
            }
        }
    }
}

fn count_detailed(item: &InfolistItem, count: &mut i32, buffers: &mut Vec<Buffer>, priority: u8) {
    for variable in item.variables.iter() {
        if match_string(&variable.name, "buffer_name") {
            if let Some(buffer_name) = match &variable.value {
                Object::Str(buffer_name) => Some(clean_string(&buffer_name)),
                _ => None,
            } {
                if priority != 1 {
                    *count += 1;
                    buffers.push(Buffer {
                        buffer: buffer_name,
                    })
                } else {
                    // Filter only slack-thread buffers
                    // A slack-thread buffer name has 4 parts separated by
                    // "."
                    // e.g "slack.workspace.#channel.03f"
                    let split = &buffer_name.split(".").collect::<Vec<&str>>();
                    if split.len() > 3 {
                        *count += 1;
                        buffers.push(Buffer {
                            buffer: buffer_name,
                        })
                    }
                }
            }
        }
    }
}
