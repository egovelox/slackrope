pub use crate::environment::{get_config, WEECHAT_HOST, WEECHAT_PASSWORD, WEECHAT_RELAY_PORT};
use anyhow::Result;
use weechat_relay_rs::commands::{Command, InitCommand, StrArgument};
use weechat_relay_rs::Connection;

pub fn init_connection() -> Result<Connection> {
    return init_connection_internal(
        &get_config::<String>(WEECHAT_HOST),
        &get_config::<String>(WEECHAT_RELAY_PORT),
        &get_config::<String>(WEECHAT_PASSWORD),
    );
}

fn init_connection_internal(host: &str, port: &str, password: &str) -> Result<Connection> {
    let weechat_host = format!("{host}:{port}");
    let stream = std::net::TcpStream::connect(weechat_host)?;
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
