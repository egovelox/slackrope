pub use crate::cli::OutputFormat;
pub use crate::models::{Buffer, Detailed, DetailedHotlist, SimpleHotlist};
pub use crate::weechat_process::{is_weechat_running, spawn_weechat_process};
use anyhow::Result;
use weechat_relay_rs::commands::{Command, InitCommand, StrArgument};
use weechat_relay_rs::Connection;

pub fn init_connection(host: &str, password: &str) -> Result<Connection> {
    let stream = std::net::TcpStream::connect(host)?;
    let mut connection = Connection { stream };
    let init_command = InitCommand::new(
        Some(StrArgument::new(password).unwrap().to_stringargument()),
        None,
        None,
    );
    let init_command: Command<InitCommand> = Command {
        id: None,
        command: init_command,
    };
    connection.send_command(&init_command)?;
    Ok(connection)
}
